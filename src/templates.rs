use tera::Tera;

pub fn load_templates() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("base", "{% block hey %}hello {% endblock hey %}"),
        (
            "pla",
            r#"{% extends "base" %}
{% block hey %}
    {% for id, plans in planning %}
        {{ id }}
        {% for plan in plans %}
            {{ plan.batch.beer.name }} System {{ plan.batch.system }}
            {{ plan.action }}
            {# plan.batch.volume | lookup #}
        {% endfor %}
    {% endfor %}
{% endblock hey %}
"#,
        ),
    ])
    .unwrap();

    tera
}
