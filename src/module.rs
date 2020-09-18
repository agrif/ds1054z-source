bobs::declare_module!(DS1054ZModule);
pub struct DS1054ZModule;

impl bobs::ModuleInfo for DS1054ZModule {
    const NAME: &'static str = "ds1054z";
    const DESCRIPTION: &'static str = "live screen grabs form a Rigol DS1054Z oscilloscope";
    const AUTHOR: &'static str = "Aaron Griffith <aargri@gmail.com>";

    fn load(r: &mut bobs::Registrar) -> Option<Self> {
        use bobs::SourceImpl;
        r.register(crate::source::ScopeSource::info());
        Some(DS1054ZModule)
    }
}
