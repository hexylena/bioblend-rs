/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LicenseMetadataModel {
    /// URL to the SPDX json details for this license
    #[serde(rename = "detailsUrl")]
    pub details_url: String,
    /// True if the entire license is deprecated
    #[serde(rename = "isDeprecatedLicenseId")]
    pub is_deprecated_license_id: bool,
    /// Indicates if the [OSI](https://opensource.org/) has approved the license
    #[serde(rename = "isOsiApproved")]
    pub is_osi_approved: bool,
    /// SPDX Identifier
    #[serde(rename = "licenseId")]
    pub license_id: String,
    /// Full name of the license
    #[serde(rename = "name")]
    pub name: String,
    /// True if this license is recommended to be used
    #[serde(rename = "recommended")]
    pub recommended: bool,
    /// Reference to the HTML format for the license file
    #[serde(rename = "reference")]
    pub reference: String,
    /// *Deprecated* - this field is generated and is no longer in use
    #[serde(rename = "referenceNumber")]
    pub reference_number: i32,
    /// Cross reference URL pointing to additional copies of the license
    #[serde(rename = "seeAlso")]
    pub see_also: Vec<String>,
    #[serde(rename = "spdxUrl")]
    pub spdx_url: String,
    /// License URL
    #[serde(rename = "url")]
    pub url: String,
}

impl LicenseMetadataModel {
    pub fn new(details_url: String, is_deprecated_license_id: bool, is_osi_approved: bool, license_id: String, name: String, recommended: bool, reference: String, reference_number: i32, see_also: Vec<String>, spdx_url: String, url: String) -> LicenseMetadataModel {
        LicenseMetadataModel {
            details_url,
            is_deprecated_license_id,
            is_osi_approved,
            license_id,
            name,
            recommended,
            reference,
            reference_number,
            see_also,
            spdx_url,
            url,
        }
    }
}


