pub struct MyCoolNotJavaButRealRustFactoryWithExtraLongNameThatDoesALotOfNiceThingsWhenYouReallyThingAboutTheTimeOfDayAndWhatReallyCanComeOfItwhenThingsGetDoneButWhatDoYouThinkAboutItHowWasYourDayByTheWayFactory<
    T: Clone + Copy + Eq + PartialEq + Ord,
    U: Clone + Copy + Eq + PartialEq + Ord,
    V: Clone + Copy + Eq + PartialEq + Ord,
    H: Clone + Copy + Eq + PartialEq + Ord,
    I: Clone + Copy + Eq + PartialEq + Ord,
    J: Clone + Copy + Eq + PartialEq + Ord,
    K: Clone + Copy + Eq + PartialEq + Ord,
    L: Clone + Copy + Eq + PartialEq + Ord,
    M: Clone + Copy + Eq + PartialEq + Ord,
> {
    t: T,
    u: U,
    v: V,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
}

impl<T: Clone + Copy + Eq + PartialEq + Ord,
U: Clone + Copy + Eq + PartialEq + Ord,
V: Clone + Copy + Eq + PartialEq + Ord,
H: Clone + Copy + Eq + PartialEq + Ord,
I: Clone + Copy + Eq + PartialEq + Ord,
J: Clone + Copy + Eq + PartialEq + Ord,
K: Clone + Copy + Eq + PartialEq + Ord,
L: Clone + Copy + Eq + PartialEq + Ord,
M: Clone + Copy + Eq + PartialEq + Ord,
> MyCoolNotJavaButRealRustFactoryWithExtraLongNameThatDoesALotOfNiceThingsWhenYouReallyThingAboutTheTimeOfDayAndWhatReallyCanComeOfItwhenThingsGetDoneButWhatDoYouThinkAboutItHowWasYourDayByTheWayFactory<T, U, V, H, I, J, K, L, M> {
    pub fn new(
        t: T,
        u: U,
        v: V,
        h: H,
        i: I,
        j: J,
        k: K,
        l: L,
        m: M,
    ) -> Self {
        Self {
            t,
            u,
            v,
            h,
            i,
            j,
            k,
            l,
            m,
        }
    }

    pub fn use_my_t(&self) -> T {
        self.t
    }

    pub fn use_my_u(&self) -> U {
        self.u
    }

    pub fn use_my_v(&self) -> V {
        self.v
    }

    pub fn use_my_h(&self) -> H{
        self.h
    }

    pub fn use_my_i(&self) -> I {
        self.i
    }


    pub fn use_my_j(&self) -> J {
        self.j
    }

    pub fn use_my_k(&self) -> K {
        self.k
    }

    pub fn use_my_l(&self) -> L {
        self.l
    }

    pub fn use_my_m(&self) -> M {
        self.m
    }


    pub fn set_my_t(&mut self, t: T) {
        self.t = t;
    }

    pub fn set_my_u(&mut self, u: U) {
        self.u = u;
    }

    pub fn set_my_v(&mut self, v: V) {
        self.v = v;
    }

    pub fn set_my_h(&mut self, h: H) {
        self.h = h;
    }


    pub fn set_my_i(&mut self, i: I) {
        self.i = i;
    }

    pub fn set_my_j(&mut self, j: J) {
        self.j = j;
    }

    pub fn set_my_k(&mut self, k: K) {
        self.k = k;
    }

    pub fn set_my_l(&mut self, l: L) {
        self.l = l;
    }

    pub fn set_my_m(&mut self, m: M) {
        self.m = m;
    }

    pub fn build(self) -> (T, U, V, H, I, J, K, L, M) {
        (self.t, self.u, self.v, self.h, self.i, self.j, self.k, self.l, self.m)
    }
}
