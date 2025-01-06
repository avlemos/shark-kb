# Intro

I wanted to do something with Rust, but I didn't really know what... But then I thought, what if I reverse engineered how my Keyboard gets its time/date set?

You see, I am a bit of a time freak, and sometimes the [Attack Shark K86](https://attackshark.com/products/attackshark-k86-wireless-mechanical-keyboard) time (or rather, seconds) on my keyboard, would get out of sync with my computer. Now, you can fix this by using their own software, click a bunch of times, and then hit Sync, but I wanted something more practical.

# How

Well, to pull this off, I used 3 tools:

1. [USBDeview](https://www.nirsoft.net/utils/usb_devices_view.html) - easy Vendor/Produdct ID, not extremely necessary
2. [USBPcap](https://desowin.org/usbpcap/) - needed to get the USB streams within Wireshark
3. [Wireshark](https://www.wireshark.org/)

In essence, I knew that there must be a protocol that the official application must be using to set the time, so all I had to do was to _sniff_ it, and try to replicate it.

After some tinkering with the filters within Wireshark to capture the right source and destination (but really, the real game changer, is to filter per protocol, in this case [HID](https://www.usb.org/hid)). Then it was a matter of using the official application, and see what I could capture in the wire.

This is what I eventually got:

```
Frame 3173804: 100 bytes on wire (800 bits), 100 bytes captured (800 bits) on interface \\.\USBPcap1, id 0
    Section number: 1
    Interface id: 0 (\\.\USBPcap1)
        Interface name: \\.\USBPcap1
        Interface description: USBPcap1
    Encapsulation type: USB packets with USBPcap header (152)
    Arrival Time: Nov  6, 2024 07:29:29.266357000 Romance Standard Time
    UTC Arrival Time: Nov  6, 2024 06:29:29.266357000 UTC
    Epoch Arrival Time: 1730885369.266357000
    [Time shift for this packet: 0.000000000 seconds]
    [Time delta from previous captured frame: 0.000044000 seconds]
    [Time delta from previous displayed frame: 14.160412000 seconds]
    [Time since reference or first frame: -5269774.783267000 seconds]
    Frame Number: 3173804
    Frame Length: 100 bytes (800 bits)
    Capture Length: 100 bytes (800 bits)
    [Frame is marked: False]
    [Frame is ignored: False]
    [Protocols in frame: usb:usbhid]
USB URB
    [Source: host]
    [Destination: 1.13.0]
    USBPcap pseudoheader length: 28
    IRP ID: 0xffff818b0dee2760
    IRP USBD_STATUS: USBD_STATUS_SUCCESS (0x00000000)
    URB Function: URB_FUNCTION_CLASS_INTERFACE (0x001b)
    IRP information: 0x00, Direction: FDO -> PDO
        0000 000. = Reserved: 0x00
        .... ...0 = Direction: FDO -> PDO (0x0)
    URB bus id: 1
    Device address: 13
    Endpoint: 0x00, Direction: OUT
        0... .... = Direction: OUT (0)
        .... 0000 = Endpoint number: 0
    URB transfer type: URB_CONTROL (0x02)
    Packet Data Length: 72
    [Response in: 3173806]
    Control transfer stage: Setup (0)
    [bInterfaceClass: HID (0x03)]
Setup Data
    bmRequestType: 0x21
        0... .... = Direction: Host-to-device
        .01. .... = Type: Class (0x1)
        ...0 0001 = Recipient: Interface (0x01)
    bRequest: SET_REPORT (0x09)
    wValue: 0x0300
        ReportID: 0
        ReportType: Feature (3)
    wIndex: 2
    wLength: 64
    Data Fragment: 28000000000000d707e80b060a1d1d00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000

```


Then it was a matter of trying to decode the payload (the Data Fragment above).

For that, I changed the date/time on my computer, so I could see some of the fields change.

```
28000000000000d707e90060a0d1200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```
```
28000000000000d707e901060a130900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```

```
28000000000000d707e901060a130a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```

So you can start to see that some bits (no pun intended) are fixed, while others change... Quite simply, it was 4 bits to define day, month, hour, minutes and seconds, and 8 bits to define the year.

Without spoiling it too much for you, 2025 in Hex is 0x7e90. You can check how the payload is built on [payload.rs](src/payload.rs).

After this was decoded, the main hurdle was that when using the [hidapi crate](https://docs.rs/hidapi/latest/hidapi/index.html), I was specifying the Vendor and Product ID that I wanted to do a Feature Report set request, but it turns out, you also need to specify which interface, in particular (this keyboard has 3 of them!).


# Future
- I do realize that this project isn't 100% Rust idiomatic, so making some changes in that direction would be nice
- Maybe this can turn into a daemon of sorts, or something that gets executed on boot, or after coming back from sleep, who knows.