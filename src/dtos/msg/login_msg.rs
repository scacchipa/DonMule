use crate::dtos::msg::header::Header;

/**
 * Protocol    Integer1Byte              Default: 0xE3
 * Size        Integer4Byte              The size of the message in bytes not including
 *                                         the header and size fields
 * Type        [Ubyte; 1]                Value of the OP LOGINREQUEST opcode
 * User Hash   [UByte; 16]               Details about user hash can be found in section 1.4
 * Client ID   [Ubyte; 4]                Default: 0x0u. The client ID that is sent on first connection
 *                                         is usually zero.
 * TCP Port    2-Byte number             Default: 4662. The TCP port used by the client, configurable
 * Tag Count   Integer4Byte              Default: 4. The number of tags following in the message
 * Name Tag    TLV (0x02; 0x01)           Default: NA. The user’s nickname (configurable in the software).
 *                                         The tag is a string tag and the tag name is an integer of value 0x1
 * Version Tag TLV (0x03; 0x11)          Default: 0x3C. The eDonkey version supported by the client.
 *                                         ) The eDonkey version
 *                                                    supported by the client.
 * Port Tag    TLV (0x03; 0X0F)          Default: 4662. The TCP port used by the client. The tag is
 *                                         an integer tag and the tag name is an integer
 *                                         of value 0x0F
 * Flags Tag 8 TLV (0x03; 0x20)          Default: 0x01. The tag is an integer tag and the tag name is 
 *                                         an integer of value 0x20
 */

struct LoginMsg {
    header: Header,
    
}