use tera::Tera;

pub fn load_templates() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("base", "{% block hey %}hello {% endblock hey %}"),
        (
            "pla",
            r#"{% extends "base" %}
{% block hey %}
    {% for plan in planning %}
        {{ plan.batch.beer.name }} System {{ plan.batch.system }} {# plan.batch.volume | lookup #}
    {% endfor %}
{% endblock hey %}
"#,
        ),
    ])
    .unwrap();

    tera
}
