mod backend;
pub fn main() {
    backend::getter::get_translations_from_location(
        r#"C:\Users\ihm1we\Developer\LeadManagement\service.dtp-lms-api.backend\01.Code\Lmt.Resources\Wizard\wizard.resx"#,
    );
}
