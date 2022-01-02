use crate::hw::gte::{VectorXY, VectorZ};
use crate::hw::gte::{VXY0, VXY1, VXY2, VZ0, VZ1, VZ2};
use crate::hw::gte::{MatrixAB, MatrixC};
use crate::hw::gte::{RT11_12, RT13_21, RT22_23, RT31_32, RT33};
use crate::hw::gte::{L11_12, L13_21, L22_23, L31_32, L33};
use crate::hw::gte::{LR11_12, LR13_21, LR22_23, LR31_32, LR33};
use crate::hw::Register;

pub struct Vector<VXY: VectorXY, VZ: VectorZ> {
    vxy: VXY,
    vz: VZ,
}

pub type V0 = Vector<VXY0, VZ0>;
pub type V1 = Vector<VXY1, VZ1>;
pub type V2 = Vector<VXY2, VZ2>;

impl<VXY: VectorXY, VZ: VectorZ> Vector<VXY, VZ> {
    /// Creates handles to a GTE vector. This caches the input values, but the
    /// load is deferred until the vector is used.
    pub fn new([x, y, z]: [i16; 3]) -> Self {
        let x = x as u32;
        let y = y as u32;
        let mut vxy = VXY::skip_load();
        vxy.assign(x | y << 16);
        let mut vz = VZ::skip_load();
        vz.assign(z);
        Self { vxy, vz }
    }

    //pub fn use(&mut self) {
    //    self.vxy.store();
    //    self.vz.store();
    //    // gte cmd goes here
    //}
}

pub struct Matrix<AB: MatrixAB, CD: MatrixAB, EF: MatrixAB, GH: MatrixAB, I: MatrixC> {
    ab: AB,
    cd: CD,
    ef: EF,
    gh: GH,
    i: I,
}

pub type RT = Matrix<RT11_12, RT13_21, RT22_23, RT31_32, RT33>;
pub type LLM = Matrix<L11_12, L13_21, L22_23, L31_32, L33>;
pub type LCM = Matrix<LR11_12, LR13_21, LR22_23, LR31_32, LR33>;

impl<AB: MatrixAB, CD: MatrixAB, EF: MatrixAB, GH: MatrixAB, I: MatrixC> Matrix<AB, CD, EF, GH, I> {
    /// Creates handles to a GTE matrix. This caches the input values, but the
    /// load is deferred until the matrix is used.
    pub fn new([r1, r2, r3]: [[i16; 3]; 3]) -> Self {
        let [r11, r12, r13] = r1.map(|r| r as u32);
        let [r21, r22, r23] = r2.map(|r| r as u32);
        let [r31, r32, r33] = r3;
        let r31 = r31 as u32;
        let r32 = r32 as u32;
        let mut ab = AB::skip_load();
        ab.assign(r11 | r12 << 16);
        let mut cd = CD::skip_load();
        cd.assign(r13 | r21 << 16);
        let mut ef = EF::skip_load();
        ef.assign(r22 | r23 << 16);
        let mut gh = GH::skip_load();
        gh.assign(r31 | r32 << 16);
        let mut i = I::skip_load();
        i.assign(r33);
        Self {
            ab, cd, ef, gh, i
        }
    }
}
