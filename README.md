# nrf-unlock

The nrf52840 revision 3 (build code Fx0) has by default a device protection mechanism. After power cycling you will not be able to (by mistake) erase its program/firmware.

This tool bypasses the mechanism tempororirly, but allows the device to re-enter the device protection mode after power cycling (on/off switch or physical disconnect).

## Install

```shell
cargo install --git https://github.com/perlindgren/nrf-unlock
```

## Use

In case your device is in protected mode, an attempt to connect to the device will give you:

```shell
Caused by:
    0: An ARM specific error occurred.
    1: An operation could not be performed because it lacked the permission to do so: erase_all
```

This problem is solved by running:

```shell
nrf-unlock
```

Notice, this will pick the first programmer reported by the operating system, and attempt to attach to it as an nRF52840_xxAA device.

It will NOT disable protection persistently, so run each time your system enters protected mode.

## TODO

The implementation is very basic:

- no device selection
- no error handling
- no status update
  
So there is lots of room for improvement.

## Resources

- [IN-141 rev 1.1](https://infocenter.nordicsemi.com/pdf/in_141_v1.1.pdf?cp=4_0_2_7)

- [probe-rs docs](https://docs.rs/probe-rs/latest/probe_rs/)