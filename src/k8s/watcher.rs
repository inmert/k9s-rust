use kube::{Client, Api};
use k8s_openapi::api::core::v1::Pod;
use anyhow::Result;

pub async fn get_client() -> Result<Client> {
    // This looks for KUBECONFIG env var or ~/.kube/config automatically
    let client = Client::try_default().await?;
    Ok(client)
}

pub async fn fetch_pods(client: &Client) -> Result<Vec<String>> {
    // Api::all looks across all namespaces
    let pods: Api<Pod> = Api::all(client.clone());

    // Use ListParams to get everything
    let list = pods.list(&kube::api::ListParams::default()).await?;

    let names = list.items
        .into_iter()
        .map(|p| {
            let name = p.metadata.name.unwrap_or_default();
            let ns = p.metadata.namespace.unwrap_or_default();
            format!("{:<15} {}", ns, name)
        })
        .collect();

    Ok(names)
}