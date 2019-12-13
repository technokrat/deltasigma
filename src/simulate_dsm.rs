// -*- coding: utf-8 -*-
// _simulateDSM_cblas.py
// Module providing a cython implementation of simulateDSM,
// based on the BLAS library.
//
// This file is part of python-deltasigma.
//
// python-deltasigma is a 1:1 Python replacement of Richard Schreier's
// MATLAB delta sigma toolbox (aka "delsigma"), upon which it is heavily based.
// The delta sigma toolbox is (c) 2009, Richard Schreier.
//
// python-deltasigma is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// LICENSE file for the licensing terms.
//
// This file is originally from `pydsm`. Little modifications have been
// performed - most prominently a bug in the ABCD matrix handling has been
// fixed. Many thanks to the original author.
//
// The original file is:
// Copyright (c) 2012, Sergio Callegari
// All rights reserved.
//
// The (future?) modifications are:
// Copyright (c) 2014, G. Venturini and the python-deltasigma contributors
//

///
/// Fast simulator for a generic delta sigma modulator using external cblas
/// =======================================================================
///

// import numpy as np
// cimport numpy as np
// import scipy as sp
// __import__('scipy.signal')
// from libc.math cimport floor, fabs

// cdef extern from "cblas.h":
//     enum CBLAS_ORDER:     CblasRowMajor, CblasColMajor
//     enum CBLAS_TRANSPOSE: CblasNoTrans, CblasTrans, CblasConjTrans
//     void cblas_dgemv(CBLAS_ORDER order, \
//         CBLAS_TRANSPOSE TransA, int M, int N,\
//         double alpha, double *A, int lda,\
//         double *X, int incX,\
//         double beta, double *Y, int incY)
//     void cblas_dcopy(int N, double *X, int incX,\
//         double *Y, int incY)

// include '_simulateDSM_helper.pxi'

use ndarray::{
    Array,
    Ix1,
    Ix2,
    s,
    stack,
    Axis,
};

use num::Complex;

pub struct ZPK {
    pub z: Array<Complex<f64>, Ix1>,
    pub p: Array<Complex<f64>, Ix1>,
    pub k: f64,
}

pub enum ModulatorType {
    ABCD(Array<f64, Ix2>),
    NTF(ZPK),
}

pub fn tf2ss(
    num: &Array<f64, Ix1>,
    den: &Array<f64, Ix1>
) -> (Array<Complex<f64>, Ix2>, Array<Complex<f64>, Ix2>, Array<Complex<f64>, Ix2>, Array<Complex<f64>, Ix2>) {
    // num, den = normalize(num, den)   # Strips zeros, checks arrays
    // nn = len(num.shape)
    // if nn == 1:
    //     num = asarray([num], num.dtype)
    // M = num.shape[1]
    // K = len(den)
    // if M > K:
    //     msg = "Improper transfer function. `num` is longer than `den`."
    //     raise ValueError(msg)
    // if M == 0 or K == 0:  # Null system
    //     return (array([], float), array([], float), array([], float),
    //             array([], float))

    // # pad numerator to have same number of columns has denominator
    // num = r_['-1', zeros((num.shape[0], K - M), num.dtype), num]

    // if num.shape[-1] > 0:
    //     D = atleast_2d(num[:, 0])

    // else:
    //     # We don't assign it an empty array because this system
    //     # is not 'null'. It just doesn't have a non-zero D
    //     # matrix. Thus, it should have a non-zero shape so that
    //     # it can be operated on by functions like 'ss2tf'
    //     D = array([[0]], float)

    // if K == 1:
    //     D = D.reshape(num.shape)

    //     return (zeros((1, 1)), zeros((1, D.shape[1])),
    //             zeros((D.shape[0], 1)), D)

    // frow = -array([den[1:]])
    // A = r_[frow, eye(K - 2, K - 1)]
    // B = eye(K - 1, 1)
    // C = num[:, 1:] - outer(num[:, 0], den[1:])
    // D = D.reshape((C.shape[0], B.shape[1]))

    // return A, B, C, D
    unimplemented!();
}

pub fn zpk2tf(zpk: &ZPK) -> (Array<f64, Ix1>, Array<f64, Ix1>) {
    // z = atleast_1d(z)
    // k = atleast_1d(k)
    // if len(z.shape) > 1:
    //     temp = poly(z[0])
    //     b = zeros((z.shape[0], z.shape[1] + 1), temp.dtype.char)
    //     if len(k) == 1:
    //         k = [k[0]] * z.shape[0]
    //     for i in range(z.shape[0]):
    //         b[i] = k[i] * poly(z[i])
    // else:
    //     b = k * poly(z)
    // a = atleast_1d(poly(p))

    // # Use real output if possible.  Copied from numpy.poly, since
    // # we can't depend on a specific version of numpy.
    // if issubclass(b.dtype.type, numpy.complexfloating):
    //     # if complex roots are all complex conjugates, the roots are real.
    //     roots = numpy.asarray(z, complex)
    //     pos_roots = numpy.compress(roots.imag > 0, roots)
    //     neg_roots = numpy.conjugate(numpy.compress(roots.imag < 0, roots))
    //     if len(pos_roots) == len(neg_roots):
    //         if numpy.all(numpy.sort_complex(neg_roots) ==
    //                      numpy.sort_complex(pos_roots)):
    //             b = b.real.copy()

    // if issubclass(a.dtype.type, numpy.complexfloating):
    //     # if complex roots are all complex conjugates, the roots are real.
    //     roots = numpy.asarray(p, complex)
    //     pos_roots = numpy.compress(roots.imag > 0, roots)
    //     neg_roots = numpy.conjugate(numpy.compress(roots.imag < 0, roots))
    //     if len(pos_roots) == len(neg_roots):
    //         if numpy.all(numpy.sort_complex(neg_roots) ==
    //                      numpy.sort_complex(pos_roots)):
    //             a = a.real.copy()

    // return b, a
    
    unimplemented!();
}

pub fn zpk2ss(zpk: &ZPK) -> (Array<Complex<f64>, Ix2>, Array<Complex<f64>, Ix2>, Array<Complex<f64>, Ix2>, Array<Complex<f64>, Ix2>) {
    let (b, a) = zpk2tf(zpk);
    tf2ss(&b, &a)
}

pub fn simulateDSM(
    u: &Array<usize, Ix2>,
    arg2: ModulatorType,
    nlev: &Array<usize, Ix2>,
    x0: &Array<f64, Ix2>,
    // int store_xn=False,
    // int store_xmax=False,
    // int store_y=False
) {
    let nu = u.shape()[0];
    let nq = nlev.shape()[0];

    let order = match arg2 {
        // This should be ensured by the type system and only be required in rust.
        ModulatorType::ABCD(ABCD) => {
            // TODO:
            assert_eq!(ABCD.shape()[1], nu + ABCD.shape()[0]);
            ABCD.shape()[0] - nq
        },
        ModulatorType::NTF(ZPK { z, .. }) => z.shape()[0],
    };

    // TODO: 
    assert_eq!(x0.shape()[0], order);

    let x0_temp = Array::zeros(x0.shape());

    // cdef np.ndarray A, B1, B2, C, D1
    let mut A;
    let mut B1;
    let mut B2;
    let mut C;
    let mut D1;
    // Build ISO Model
    // note that B=hstack((B1, B2))
    match arg2 {
        ModulatorType::ABCD(ABCD) => {
            A = ABCD.slice(s![0..order, 0..order]);
            B1 = ABCD.slice(s![0..order, order..order + nu]);
            B2 = ABCD.slice(s![0..order, order + nu..order + nu + nq]);
            C = ABCD.slice(s![order..order + nq, 0..order]);
            D1 = ABCD.slice(s![order..order + nq, order..order + nu]);
        },
        ModulatorType::NTF(ZPK { z, p, .. }) => {
            // Seek a realization of -1/H
            let (A, B2, mut C, D2) = zpk2ss(&ZPK { z: p, p: z, k: -1.0 });
            C = C.map(|c| c.re.into());
            // Transform the realization so that C = [1 0 0 ...]
            let Sinv = sp.linalg.orth(
                stack![Axis(1), (C.t(), Array::eye(order))]
            ) / np.linalg.norm(C);
            let S = sp.linalg.inv(Sinv);
            C = np.dot(C, Sinv);
            if C[0, 0] < 0:
                S = -S
                Sinv = -Sinv
            A = np.asarray(S.dot(A).dot(Sinv), dtype=np.float64, order='C')
            B2 = np.asarray(np.dot(S, B2), dtype=np.float64, order='C')
            C = np.asarray(np.hstack(([[1.]], np.zeros((1,order-1)))),\
                dtype=np.float64, order='C')
            // C=C*Sinv;
            // D2 = 0;
            // !!!! Assume stf=1
            B1 = -B2
            D1 = np.asarray(1., dtype=np.float64)
            B = np.hstack((B1, B2))
        }
    }
    // else:
    //     // Seek a realization of -1/H
    //     A, B2, C, D2 = sp.signal.zpk2ss(ntf_p, ntf_z, -1)
    //     C=C.real
    //     // Transform the realization so that C = [1 0 0 ...]
    //     Sinv = sp.linalg.orth(np.hstack((np.transpose(C), np.eye(order))))/ \
    //         np.linalg.norm(C)
    //     S = sp.linalg.inv(Sinv)
    //     C = np.dot(C, Sinv)
    //     if C[0, 0] < 0:
    //         S = -S
    //         Sinv = -Sinv
    //     A = np.asarray(S.dot(A).dot(Sinv), dtype=np.float64, order='C')
    //     B2 = np.asarray(np.dot(S, B2), dtype=np.float64, order='C')
    //     C = np.asarray(np.hstack(([[1.]], np.zeros((1,order-1)))),\
    //         dtype=np.float64, order='C')
    //     // C=C*Sinv;
    //     // D2 = 0;
    //     // !!!! Assume stf=1
    //     B1 = -B2
    //     D1 = np.asarray(1., dtype=np.float64)
    //     //B = np.hstack((B1, B2))

    // // N is number of input samples to deal with
    // cdef int N = c_u.shape[1]
    // // v is output vector
    // cdef np.ndarray v = np.empty((nq, N), dtype=np.float64)
    // cdef np.ndarray y
    // if store_y:
    //     // Need to store the quantizer input
    //     y = np.empty((nq, N), dtype=np.float64)
    // else:
    //     y = np.empty((0,0), dtype=np.float64)
    // cdef np.ndarray xn
    // if store_xn:
    //     // Need to store the state information
    //     xn = np.empty((order, N), dtype=np.float64)
    // cdef np.ndarray xmax
    // if store_xmax:
    //     // Need to keep track of the state maxima
    //     xmax = np.abs(c_x0)
    // else:
    //     xmax = np.empty(0, dtype=np.float64)

    // // y0 is output before the quantizer
    // cdef np.ndarray y0 = np.empty(nq, dtype=np.float64)

    // cdef int i
    // for i in xrange(N):
    //     // Compute y0 = np.dot(C, c_x0) + np.dot(D1, u[:, i])
    //     cblas_dgemv(CblasRowMajor, CblasNoTrans, nq, order,\
    //         1.0, dbldata(C), order, \
    //         dbldata(c_x0), 1, \
    //         0.0, dbldata(y0), 1)
    //     cblas_dgemv(CblasRowMajor, CblasNoTrans, nq, nu,\
    //         1.0, dbldata(D1), nu, \
    //         dbldata(c_u)+i, N, \
    //         1.0, dbldata(y0), 1)
    //     if store_y:
    //         //y[:, i] = y0[:]
    //         cblas_dcopy(nq, dbldata(y0), 1,\
    //         dbldata(y)+i, N)
    //     ds_quantize(nq, dbldata(y0), 1, \
    //         intdata(c_nlev), 1, \
    //         dbldata(v)+i, N)
    //     // Compute c_x0 = np.dot(A, c_x0) +
    //     //   np.dot(B, np.vstack((u[:, i], v[:, i])))
    //     cblas_dgemv(CblasRowMajor, CblasNoTrans, order, order,\
    //         1.0, dbldata(A), order, \
    //         dbldata(c_x0), 1,\
    //         0.0, dbldata(c_x0_temp), 1)
    //     cblas_dgemv(CblasRowMajor, CblasNoTrans, order, nu,\
    //         1.0, dbldata(B1), nu, \
    //         dbldata(c_u)+i, N, \
    //         1.0, dbldata(c_x0_temp), 1)
    //     cblas_dgemv(CblasRowMajor, CblasNoTrans, order, nq,\
    //         1.0, dbldata(B2), nq, \
    //         dbldata(v)+i, N, \
    //         1.0, dbldata(c_x0_temp), 1)
    //     // c_x0[:,1] = c_x0_temp[:,1]
    //     cblas_dcopy(order, dbldata(c_x0_temp), 1,\
    //         dbldata(c_x0), 1)
    //     if store_xn:
    //         // Save the next state
    //         //xn[:, i] = c_x0
    //         cblas_dcopy(order, dbldata(c_x0), 1,\
    //         dbldata(xn)+i, N)
    //     if store_xmax:
    //         // Keep track of the state maxima
    //         // xmax = np.max((np.abs(x0), xmax), 0)
    //         track_vabsmax(order, dbldata(xmax), 1,\
    //             dbldata(c_x0), 1)
    // if not store_xn:
    //     xn = c_x0
    // return v.squeeze(), xn.squeeze(), xmax, y.squeeze()

}