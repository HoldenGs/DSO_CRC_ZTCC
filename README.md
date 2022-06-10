# DSO_CRC_ZTCC
Distance Spectrum Optimized (DSO) Cyclic Redundancy Check (CRC) for Zero-Terminated Convolutional Codes (ZTCCs)

We use a formula that uses the distance-spectrum of a given data length and polynomial degree to find optimized convolutional codes which can be used to optimize for low-error while also being relatively low-power.

This repository represents a first pass on porting the code from Dr. Chung-Yu Lou's paper into a performant Rust implementation. A MATLAB implementation created by Dr. Hengjie Yang is used for reference, which can be found [here](https://github.com/hengjie-yang/DSO_CRC_Design_for_ZTCCs/blob/master/check_divisible_by_distance.m).
