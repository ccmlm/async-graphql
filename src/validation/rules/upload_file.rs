use crate::validation::context::ValidatorContext;
use crate::validation::visitor::Visitor;
use graphql_parser::query::OperationDefinition;

#[derive(Default)]
pub struct UploadFile;

impl<'a> Visitor<'a> for UploadFile {
    fn enter_operation_definition(
        &mut self,
        ctx: &mut ValidatorContext<'a>,
        operation_definition: &'a OperationDefinition,
    ) {
        match operation_definition {
            OperationDefinition::Query(query) => {
                for var in &query.variable_definitions {
                    if let Some(ty) = ctx.registry.basic_type_by_parsed_type(&var.var_type) {
                        if ty.name() == "Upload" {
                            ctx.report_error(
                                vec![var.position],
                                "The Upload type is only allowed to be defined on a mutation",
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
}