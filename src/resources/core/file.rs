use crate::resources::common::object::Object;
use crate::util::List;
use crate::resources::core::filelink::FileLink;
use std::collections::HashMap;
use crate::{StripeService, Client};
use crate::resources::common::path::{UrlPath, StripePath};
use reqwest::multipart::Form;
use std::path::Path;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct File {
    pub id: String,
    pub object: Object,
    pub created: i64,
    pub filename: String,
    pub links: List<FileLink>,
    pub purpose: FilePurpose,
    pub size: i64,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub url: Option<String>,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum FilePurpose {
    BusinessLogo,
    FinanceReportRun,
    FoundersStockDocument,
    DisputeEvidence,
    IdentityDocument,
    CustomerSignature,
    PciDocument,
    TaxDocumentUserUpload,
    SigmaScheduledQuery,
}

impl Into<Cow<'static, str>> for FilePurpose {
    fn into(self) -> Cow<'static, str> {
        match self {
            FilePurpose::BusinessLogo => "business_logo",
            FilePurpose::DisputeEvidence => "dispute_evidence",
            FilePurpose::IdentityDocument => "identity_document",
            FilePurpose::CustomerSignature => "customer_signature",
            FilePurpose::PciDocument => "pci_document",
            FilePurpose::TaxDocumentUserUpload => "tax_document_user_upload",
            FilePurpose::SigmaScheduledQuery => "sigma_scheduled_query",
            FilePurpose::FinanceReportRun => "finance_report_run",
            FilePurpose::FoundersStockDocument => "founders_stock_document"
        }.into()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum FileType {
    PDF,
    XML,
    JPG,
    PNG,
    CSV,
    TSV,
    XLS,
    XLSX,
    DOCX,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileLinkDataParam {
    pub create: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>
}

impl StripeService for File {
    fn uri(&self, path: UrlPath, param: &StripePath) -> String {
        format!("https://files.stripe.com/v1/{}{}", path, param)
    }
}

impl StripeService for FileLinkDataParam {}

impl File {

    pub fn create<B: serde::Serialize + StripeService, P: AsRef<Path>>(client: &Client, path: P, purpose: FilePurpose, file_link: B) -> crate::Result<Self> {
        let form= Form::new()
            .text("purpose", purpose)
            .file("file", path.as_ref())?;

        client.upload(UrlPath::File, &StripePath::default(), file_link, form)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::File, &StripePath::default().param(id), Self::object())
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::File, &StripePath::default(), param)
    }

}