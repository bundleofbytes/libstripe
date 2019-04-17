use crate::resources::common::object::Object;
use crate::util::List;
use crate::resources::core::filelink::FileLink;
use std::fmt;
use std::collections::HashMap;
use crate::{StripeService, Client};
use crate::resources::common::path::{UrlPath, StripePath};
use reqwest::multipart::Form;
use std::path::Path;

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
pub enum FilePurpose {
    #[serde(rename = "business_logo")]
    BusinessLogo,
    #[serde(rename = "finance_report_run")]
    FinanceReportRun,
    #[serde(rename = "founders_stock_document")]
    FoundersStockDocument,
    #[serde(rename = "dispute_evidence")]
    DisputeEvidence,
    #[serde(rename = "identity_document")]
    IdentityDocument,
    #[serde(rename = "customer_signature")]
    CustomerSignature,
    #[serde(rename = "pci_document")]
    PciDocument,
    #[serde(rename = "tax_document_user_upload")]
    TaxDocumentUserUpload,
    #[serde(rename = "sigma_scheduled_query")]
    SigmaScheduledQuery,
}

impl fmt::Display for FilePurpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           FilePurpose::BusinessLogo => write!(f, "business_logo"),
           FilePurpose::DisputeEvidence => write!(f, "dispute_evidence"),
           FilePurpose::IdentityDocument => write!(f, "identity_document"),
           FilePurpose::CustomerSignature => write!(f, "customer_signature"),
           FilePurpose::PciDocument => write!(f, "pci_document"),
           FilePurpose::TaxDocumentUserUpload => write!(f, "tax_document_user_upload"),
           FilePurpose::SigmaScheduledQuery => write!(f, "sigma_scheduled_query"),
           FilePurpose::FinanceReportRun => write!(f, "finance_report_run"),
           FilePurpose::FoundersStockDocument => write!(f, "founders_stock_document")
       }
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
            .text("purpose", format!("{}", purpose))
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