use crate::helpers::{
    camelcase, component_path, fetch_patterns, has, is_empty, is_http_code_success, json,
    mixedcase, not_empty, patterns, sanitize, shoutysnakecase, snakecase,
};
use anyhow::{Context, Result};
use handlebars::Handlebars;
use log;
use std::{fs::File, path::Path};

pub struct OpenApiGenerator<'a> {
    handlebars: Handlebars<'a>,
    specs: serde_yaml::Value,
}

impl<'a> OpenApiGenerator<'a> {
    pub fn new(specs_path: impl AsRef<Path>) -> Result<Self> {
        let mut openapi_generator = Self {
            handlebars: Handlebars::new(),
            specs: Self::parse_specification(&specs_path.as_ref())?,
        };
        openapi_generator
            .register_partials()
            .context("Failed to register partials")?;
        openapi_generator.register_helpers();

        let specs = openapi_generator
            .specs
            .as_mapping_mut()
            .context("specification is not a mapping")?;

        specs.insert(
            serde_yaml::Value::String("openapi_generator_version".to_string()),
            serde_yaml::Value::String(env!("CARGO_PKG_VERSION").to_string()),
        );

        Ok(openapi_generator)
    }

    fn parse_specification(specs_path: &Path) -> Result<serde_yaml::Value> {
        let specs_string = std::fs::read_to_string(&specs_path).context(format!(
            "Cannot read specification file `{}`",
            specs_path.display()
        ))?;
        serde_yaml::from_str(&specs_string).context(format!(
            "Cannot parse specification file `{}`",
            specs_path.display()
        ))
    }

    fn register_helpers(&mut self) {
        self.handlebars
            .register_helper("camelcase", Box::new(camelcase));
        self.handlebars
            .register_helper("patterns", Box::new(patterns));
        self.handlebars
            .register_helper("snakecase", Box::new(snakecase));
        self.handlebars
            .register_helper("shoutysnakecase", Box::new(shoutysnakecase));
        self.handlebars
            .register_helper("mixedcase", Box::new(mixedcase));
        self.handlebars
            .register_helper("component_path", Box::new(component_path));
        self.handlebars
            .register_helper("sanitize", Box::new(sanitize));
        self.handlebars.register_helper("has", Box::new(has));
        self.handlebars
            .register_helper("fetch_patterns", Box::new(fetch_patterns));
        self.handlebars
            .register_helper("is_empty", Box::new(is_empty));
        self.handlebars
            .register_helper("not_empty", Box::new(not_empty));
        self.handlebars.register_helper("json", Box::new(json));
        self.handlebars
            .register_helper("is_http_code_success", Box::new(is_http_code_success));
    }

    fn register_partials(&mut self) -> Result<()> {
        let partials = &[
            ("data_type", include_str!("templates/partials/data_type.rs")),
            ("example", include_str!("templates/partials/example.rs")),
            (
                "operation_examples",
                include_str!("templates/partials/operation_examples.rs"),
            ),
            (
                "operation_types",
                include_str!("templates/partials/operation_types.rs"),
            ),
            (
                "parameter_type",
                include_str!("templates/partials/parameter_type.rs"),
            ),
            (
                "schema_example",
                include_str!("templates/partials/schema_example.rs"),
            ),
            ("schema", include_str!("templates/partials/schema.rs")),
            (
                "subtypes_example",
                include_str!("templates/partials/subtypes_example.rs"),
            ),
            ("subtypes", include_str!("templates/partials/subtypes.rs")),
            (
                "test_operation_client",
                include_str!("templates/partials/test_operation_client.rs"),
            ),
            (
                "validation",
                include_str!("templates/partials/validation.rs"),
            ),
        ];

        for (template_name, template_string) in partials {
            self.handlebars
                .register_template_string(template_name, template_string)
                .context(format!("Cannot register partial `{}`", template_name))?;
            log::info!("new partial registered: {} ", template_name);
        }
        Ok(())
    }

    pub fn render(&mut self, output_path: impl AsRef<Path>) -> Result<()> {
        let template_string = include_str!("templates/oapi.rs");
        self.handlebars
            .register_template_string("templates/oapi.rs", template_string)
            .context("Cannot register template templates/oapi.rs")?;
        log::info!("new template registered: templates/oapi.rs");
        let mut output_file = File::create(&output_path)?;
        self.handlebars
            .render_to_write("templates/oapi.rs", &self.specs, &mut output_file)
            .context(format!(
                "Failed to render template templates/oapi.rs at `{}`",
                output_path.as_ref().display()
            ))?;
        log::info!(
            "render templates/oapi.rs to {}",
            output_path.as_ref().display()
        );
        Ok(())
    }
}
