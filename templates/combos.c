
{% for combo in combos.iter() %}
const uint16_t PROGMEM {{combo.name}}_combo[] = { {{ combo.triggers }} };
{% endfor %}