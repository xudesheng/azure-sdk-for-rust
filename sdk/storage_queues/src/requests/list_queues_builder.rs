use crate::responses::*;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::{unfold, Stream};
use http::method::Method;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListQueuesBuilder<'a> {
    storage_client: &'a StorageClient,
    prefix: Option<Prefix>,
    next_marker: Option<NextMarker>,
    max_results: Option<MaxResults>,
    include_metadata: bool,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> ListQueuesBuilder<'a> {
    pub(crate) fn new(storage_client: &'a StorageClient) -> Self {
        Self {
            storage_client,
            prefix: None,
            next_marker: None,
            max_results: None,
            include_metadata: false,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        prefix: Prefix => Some(prefix),
        next_marker: NextMarker => Some(next_marker),
        max_results: MaxResults => Some(max_results),
        include_metadata: bool => include_metadata,
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<ListQueuesResponse> {
        let mut url = self
            .storage_client
            .storage_account_client()
            .queue_storage_url()
            .to_owned();

        url.query_pairs_mut().append_pair("comp", "list");

        self.prefix.append_to_url_query(&mut url);
        self.next_marker.append_to_url_query(&mut url);
        self.max_results.append_to_url_query(&mut url);

        if self.include_metadata {
            url.query_pairs_mut().append_pair("include", "metadata");
        }

        self.timeout.append_to_url_query(&mut url);
        AppendToUrlQuery::append_to_url_query(&self.timeout, &mut url);

        let mut request = self
            .storage_client
            .prepare_request(url.as_str(), Method::GET, None)?;
        request.add_optional_header(&self.client_request_id);

        let response = self
            .storage_client
            .storage_account_client()
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }

    pub fn stream(self) -> impl Stream<Item = azure_core::Result<ListQueuesResponse>> + 'a {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            NextMarker(NextMarker),
        }

        unfold(Some(States::Init), move |next_marker: Option<States>| {
            let req = self.clone();
            async move {
                let response = match next_marker {
                    Some(States::Init) => req.execute().await,
                    Some(States::NextMarker(next_marker)) => {
                        req.next_marker(next_marker).execute().await
                    }
                    None => return None,
                };

                // the ? operator does not work in async move (yet?)
                // so we have to resort to this boilerplate
                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                let next_marker = response
                    .next_marker()
                    .as_ref()
                    .map(|next_marker| States::NextMarker(next_marker.to_owned()));

                Some((Ok(response), next_marker))
            }
        })
    }
}