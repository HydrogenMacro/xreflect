impl :: xreflect :: Reflect for Enemy
{
    fn amount_of_fields(& self) -> usize { 1usize } fn
    try_get_index_of_field(& self, field_name : & str) -> Result < usize, ::
    xreflect :: ReflectError >
    {
        match field_name
        {
            "health" => Ok(0usize), _ =>
            Err(:: xreflect :: ReflectError :: FieldNotFound)
        }
    } fn try_get_field_at < T : 'static > (& self, field_index : usize) ->
    Result < & T, :: xreflect :: ReflectError >
    {
        match field_index
        {
            0usize =>
            {
                let Self { ref health } = self;
                (health as & dyn :: core :: any :: Any).downcast_ref :: < T >
                ().ok_or(:: xreflect :: ReflectError :: WrongType)
            }, _ => Err(:: xreflect :: ReflectError :: FieldNotFound)
        }
    } fn try_get_field_mut_at < T : 'static >
    (& mut self, field_index : usize) -> Result < & mut T, :: xreflect ::
    ReflectError >
    {
        match field_index
        {
            0usize =>
            {
                let Self { ref mut health } = self;
                (health as & mut dyn :: core :: any :: Any).downcast_mut :: <
                T > ().ok_or(:: xreflect :: ReflectError :: WrongType)
            }, _ => Err(:: xreflect :: ReflectError :: FieldNotFound)
        }
    } fn try_get_type_of_field_at(& self, field_index : usize) -> Result < ::
    core :: any :: TypeId, :: xreflect :: ReflectError >
    {
        match field_index
        {
            0usize => Ok(:: core :: any :: TypeId :: of :: < u8 > ()), _ =>
            Err(:: xreflect :: ReflectError :: FieldNotFound)
        }
    }
}