uint16_t get_combo_term(uint16_t index, combo_t *combo) {
    switch (index) {
{% for combo in combos.iter() %}
        case {{ combo.name}}:
            return COMBO_TERM + {{ combo.timeout_ms }};
{% endfor %}                    
        // Regular combos, slightly relaxed
        default:
            return COMBO_TERM + 25;
   }
}