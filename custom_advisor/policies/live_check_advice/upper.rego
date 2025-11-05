package live_check_advice

import rego.v1

# If the annotation is set to "upper", check that the attribute value is uppercase
deny contains make_advice(advice_type, advice_level, advice_context, message) if {
    input.sample.attribute.value != null
    input.registry_attribute.annotations.live_check.case == "upper"
    is_string(input.sample.attribute.value)
    not input.sample.attribute.value == upper(input.sample.attribute.value)
    advice_type := "invalid_value_case"
    advice_level := "violation"
    advice_context := {"attribute_name": input.sample.attribute.name,
                       "attribute_value": input.sample.attribute.value}
    message := sprintf(
        "Value '%s' on attribute '%s' must be uppercase",
        [input.sample.attribute.value, input.sample.attribute.name],
    )
}

make_advice(advice_type, advice_level, advice_context, message) := {
	"type": "advice",
	"advice_type": advice_type,
	"advice_level": advice_level,
	"advice_context": advice_context,
	"message": message,
}
