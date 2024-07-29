pub(crate) struct EnumData {
	generics: Vec<GenericType>,
	variants: Vec<EnumVariantData>
}
pub(crate) struct EnumVariantData {
	name: String,
	enum_type: EnumVariantType
}
pub(crate) enum EnumVariantType {
	Unit,
	Tuple(Vec<TupleEntry>),
	Struct(Vec<RecordEntry>)
}
pub(crate) struct RecordEntry(String, syn::Type);
pub(crate) struct TupleEntry(syn::Type);
pub(crate) enum GenericType {
	Lifetime { name: String, bounds: String },
	Type { name: String, bounds: String },
	Const { name: String, const_type: String }
}