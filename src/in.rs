trait VarIn {
    fn inGet<Type, Past: Stack>(
        &self: Past,
        &var: Var<Type>,
    ) -> Stack<Type, Past>;
}
