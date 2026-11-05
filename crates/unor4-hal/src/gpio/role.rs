//! Signal roles: the second parameter of the channel pin traits
//! (`SciPin<SCI2, Txd>`).

macro_rules! roles {
    ( $( $ty:ident: $doc:literal; )* ) => {
        $(
            #[doc = $doc]
            #[derive(Clone, Copy, Debug)]
            pub struct $ty;
        )*
    };
}

roles! {
    Txd: "SCI `TXDn` (also `MOSIn`/`SDAn` in the SCI's simple SPI/I²C modes).";
    Rxd: "SCI `RXDn` (also `MISOn`/`SCLn` in the SCI's simple SPI/I²C modes).";
    Sck: "SCI `SCKn`.";
    Cts: "SCI `CTSn_RTSn`/`SSn`.";
    Sda: "IIC `SDAn`.";
    Scl: "IIC `SCLn`.";
    Mosi: "SPI `MOSI`.";
    Miso: "SPI `MISO`.";
    Rspck: "SPI `RSPCK`.";
    Ssl0: "SPI `SSL0`.";
    Ssl1: "SPI `SSL1`.";
    Ssl2: "SPI `SSL2`.";
    Ssl3: "SPI `SSL3`.";
    CanTx: "CAN `CTXn`.";
    CanRx: "CAN `CRXn`.";
}
