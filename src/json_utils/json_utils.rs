use crate::json_utils::json_field_missing_error::JSONFieldMissingError;

pub fn validate_keys_in_json(
    json: &serde_json::Value,
    required_props: Vec<&str>
) -> Result<(), JSONFieldMissingError> {
    for property in required_props {
        if json[property].is_null() {
            return Err(JSONFieldMissingError { field_name: String::from(property) });
        }
    }

    Ok(())
}