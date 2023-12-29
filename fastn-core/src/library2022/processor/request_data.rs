pub fn process(
    value: ftd::ast::VariableValue,
    kind: ftd::interpreter::Kind,
    doc: &ftd::interpreter::TDoc,
    req_config: &fastn_core::RequestConfig,
) -> ftd::interpreter::Result<ftd::interpreter::Value> {
    let mut data = req_config.request.query().clone();

    for (name, param_value) in req_config.named_parameters.iter() {
        let json_value =
            param_value
                .to_serde_value()
                .ok_or(ftd::ftd2021::p1::Error::ParseError {
                    message: format!("ftd value cannot be parsed to json: name: {name}"),
                    doc_id: doc.name.to_string(),
                    line_number: value.line_number(),
                })?;
        data.insert(name.to_string(), json_value);
    }

    match req_config.request.body_as_json() {
        Ok(Some(b)) => {
            data.extend(b);
        }
        Ok(None) => {}
        Err(e) => {
            return ftd::interpreter::utils::e2(
                format!("Error while parsing request body: {e:?}"),
                doc.name,
                value.line_number(),
            )
        }
    }

    data.extend(
        req_config
            .extra_data
            .iter()
            .map(|(k, v)| (k.to_string(), serde_json::Value::String(v.to_string()))),
    );

    doc.from_json(&data, &kind, &value)
}
