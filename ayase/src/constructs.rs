pub struct Db<S: Site, L: Layout>(PhantomData<S>, PhantomData<L>);

impl Default for Db {
    fn default() -> Self {
        Db(PhantomData, PhantomData)
    }
}