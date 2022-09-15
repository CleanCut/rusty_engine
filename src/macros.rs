#[macro_export]
macro_rules! repository {
    (
        $name:ident<$entity:ident>
    ) => {
        #[derive(Debug, Default)]
        pub struct $name($crate::traits::EntityMap<$entity>);
        impl $crate::traits::Reposit<$entity> for $name {
            fn inner(&self) -> &$crate::traits::EntityMap<$entity> {
                &self.0
            }
            fn inner_mut(&mut self) -> &mut $crate::traits::EntityMap<$entity> {
                &mut self.0
            }
        }
    };
}
