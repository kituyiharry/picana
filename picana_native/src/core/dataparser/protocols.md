Using a CAN DBC for this CAN ID, you can get the 'scaled engineering values':

## What is DBC

A CAN bus DBC file is simply a database format structured around CAN Messages (e.g. EEC1) and Signals (e.g. RPM).
The DBC format is proprietary, but a number of online DBC wikis detail the basic concepts.
The core of a DBC file consists of CAN Messages and their Signal details;

## WHAT IS SAE J1939?

1939 is a set of standards defining how ECUs communicate, e.g. in heavy-duty vehicles

Applications include trucks, buses, agriculture, maritime, construction and more
It is based on the high-speed Controller Area Network (CAN) bus, ISO11898
J1939 is standardized, i.e. ECUs can communicate across manufacturers

As evident from our intro to CAN bus, most vehicles today use the Controller Area Network (CAN) for ECU communication.
However, CAN bus only provides a "tool" for communication (like a telephone) - not the "language" for conversation.

In most commercial vehicles, this language is the SAE J1939 standard defined by the Society of Automotive Engineers (SAE).
In more technical terms, J1939 provides a higher layer protocol (HLP) using CAN as the "physical layer" basis.
n contrast, passenger cars typically rely on manufacturer specific protocols.
Heavy-duty vehicles (e.g. trucks and buses) is one of the most well-known applications.
However, several other key industries leverage SAE J1939 today either directly or via derived standards (e.g. ISO 11783, MilCAN, NMEA 2000, FMS):

For this reason, a good understanding of the J1939 protocol is core in e.g. building fleet management systems.

# KEY CHARACTERISTICS OF J1939

To dig a bit deeper, let's look at some key characteristics of SAE J1939:

-> Speed: The speed is typically 250 kbit/s, though recently with support for 500 kbit/s
-> Extended: J1939 uses CAN 2.0B, i.e. an extended 29 bit identifier
-> PGNs: Messages are identified by PGNs (Parameter Group Numbers), which comprise 18 of the 29 bit identifier
-> SPNs: A PGN contains a number of SPNs (Suspect Parameter Numbers) in the 8 data bytes reflecting parameters (e.g. RPM)
-> Messages: Messages can be broadcast, sent peer-to-peer or be requested
-> Reserved: J1939 includes a large range of standard PGNs, though PGNs 00FF00 through 00FFFF are reserved for proprietary use
-> Special Values: A data byte of 0xFF (255) reflects N/A data, while 0xFE (254) reflects an error
-> Multibyte: Multibyte variables are sent least significant byte first (Intel byte order)
-> Multi-Packet: J1939 supports PGNs with up to 1785 bytes using a transport protocol

: Each J1939 message is identified via a PGN and contains 8 data bytes, split into parameters called -> SPNs

## PARAMETER GROUP NUMBER (PGN)

-> A PGN is a unique ID for looking up the function of a J1939 message and the associated data parameters (i.e. the SPNs)

You cannot match PGNs vs the full 29 bit CAN identifier. Instead, you need to separate out the 18 bit PGN as below.

**NB**: Some say PGN is 16 bits others say 18bits....take this into account

CAN MESSAGE -> [ 29 bit identifier ] .. [ 64 bit data field ] => J19939 -> [ 18|16 Bit PGN ] .. [ SPN .. groups ]

!(PGN-SPN)[https://canlogger1000.csselectronics.com/img/SAE-J1939-PGN-SPN-Message-Structure-Identifier-CAN-Bus.png]

Let's say we logged a J1939 message with ID 0x0CF00401.

- Here, the PGN starts at bit 9, with length 18 (indexed from 1).
- The outcome is the PGN 0x0F004 or in decimal 61444. Looking this up in the SAE J1939-71 documentation we see that it’s the “Electronic Engine Controller 1 - EEC1”.

- Further, the document will have details on the PGN including priority, transmission rate and a list of the associated SPNs - cf. the example below.

- For this PGN, there are seven SPNs (e.g. Engine Speed, RPM), each of which can be looked up in the J1939-71 documentation for further details.

# Is it That Simple ?

Not completely: The above is a bit simplified as the J1939 29-bit identifier can be broken further down.

- Specifically, the ID comprises the Priority (3 bits), PGN (18 bits) and Source Address (8 bits).

- Further, the PGN can be broken into four parts: Reserved (1 bit), Data Page (1 bit), PDU Format (8 bits) and PDU Specific (8 bits).

## SUSPECT PARAMETER NUMBER (SPN)

-> The SPNs of a J1939 message reflect data parameters - such as speed and RPM

In practice, you won't sit and lookup the J1939-71 PDF!

a resolution on how to scale the offset is also provided

Rather, most utilize software that can load J1939 "DBC" files to convert logged or streamed J1939 data.

In a DBC context, PGNs are often called “Messages” and SPNs are called “Signals”. For more on this, check out our DBC conversion article which uses SAE J1939 as a case example.

Most J1939 messages are broadcast to the CAN bus, but some need to be requested (e.g. some J1939 diagnostic trouble codes).
This is achieved using the ‘request message’ (PGN 59904), which is the only J1939 message with only 3 bytes of data.
It has priority 6, a variable transmit rate and can either be sent as a global or specific address request.
The data bytes 1-3 should contain the requested PGN (Intel byte order). Examples of requested J1939 messages include the diagnostic messages (DM).
As for OBD2, you can use our the transmit list of e.g. our CANedge to set up SAE J1939 request messages.

These are referred to as J1939 multi-frame or multi-packet messages. The J1939 protocol specifies how to deconstruct, transfer and reassemble the packets - a process referred to as the Transport Protocol (cf. J1939-21). Two types exist:

-- The Connection Mode (intended for a specific device)
-- The BAM (Broadcast Announce Message) which is intended for the entire network.

In simple terms, the BAM works by the transmitting ECU sending an initial BAM packet to set up the transfer.

The BAM specifies the multi-packet PGN identifier as well as the number of data bytes and packets to be sent.
It is then followed by up to 255 packets of data. Each of the 255 packets use the first data byte to specify the sequence number (1 up to 255), followed by 7 bytes of data.
The max number of bytes per multi-packet message is therefore 7 bytes x 255 = 1785 bytes. The final packet will contain at least one byte of data, followed by unused bytes set to FF. In the BAM type scenario, the time between messages is 50-200 ms.
Finally, a conversion software can reassemble the multiple entries of 7 data bytes into a single string and handle it according to the multi-packet PGN and SPN specifications.

## HOW Does DBC Conversion Work

()![https://canlogger1000.csselectronics.com/img/CAN-DBC-File-Format-Explained-Intro-Basics_2.png]

A few comments on the DBC structure above:

-> Each CAN Message contains 1+ CAN Signals
-> The 'DBC Message ID' adds 3 extra bits for 29 bit CAN IDs as an 'extended CAN ID' syntax
-> Bit positions start at 0 - above example Signal starts after the 3rd byte with a 2-byte length
-> The Sender and Receiver reflect sending & receiving nodes
-> More info (e.g. descriptions, value tables) can be added later in the DBC file

In short, the CAN DBC file is a standardized format for storing the rules for converting raw CAN bus data.
For standardized cases like the SAE J1939, you can use the same DBC across many vehicles to convert most data.
However, for proprietary CAN protocol systems (e.g in cars), typically only the OEM will have the conversion rules.
If you do not have access to the rules, you may be able to reverse engineer some of the information.

To convert a single CAN frame observation, we extract the 2 date bytes after byte 3 (e.g. 68 13), swap order (little endian) and convert to decimal (4,968).

Stream J1939 Data Convert RPM from Transit Bus J1939 Heavy Duty DBC
To "scale" this RPM data to human readable form, the J1939 standard dictates a linear conversion as below:

[Scaled Value] = [Offset] + [Scale] x [Raw Decimal Value]

more at https://www.csselectronics.com/screen/page/dbc-database-can-bus-conversion-wireshark-j1939-example/language/en
