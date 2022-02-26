/// Source of this list:
/// https://ourcodeworld.com/articles/read/1536/list-of-free-functional-public-stun-servers-2021
const STUN_SERVERS: [&str; 137] = [
    "stun.l.google.com:19302",
    "iphone-stun.strato-iphone.de:3478",
    "numb.viagenie.ca:3478",
    "stun.12connect.com:3478",
    "stun.12voip.com:3478",
    "stun.1und1.de:3478",
    "stun.3cx.com:3478",
    "stun.acrobits.cz:3478",
    "stun.actionvoip.com:3478",
    "stun.advfn.com:3478",
    "stun.altar.com.pl:3478",
    "stun.antisip.com:3478",
    "stun.avigora.fr:3478",
    "stun.bluesip.net:3478",
    "stun.cablenet-as.net:3478",
    "stun.callromania.ro:3478",
    "stun.callwithus.com:3478",
    "stun.cheapvoip.com:3478",
    "stun.cloopen.com:3478",
    "stun.commpeak.com:3478",
    "stun.cope.es:3478",
    "stun.counterpath.com:3478",
    "stun.counterpath.net:3478",
    "stun.dcalling.de:3478",
    "stun.demos.ru:3478",
    "stun.dus.net:3478",
    "stun.easycall.pl:3478",
    "stun.easyvoip.com:3478",
    "stun.ekiga.net:3478",
    "stun.epygi.com:3478",
    "stun.etoilediese.fr:3478",
    "stun.faktortel.com.au:3478",
    "stun.freecall.com:3478",
    "stun.freeswitch.org:3478",
    "stun.freevoipdeal.com:3478",
    "stun.gmx.de:3478",
    "stun.gmx.net:3478",
    "stun.halonet.pl:3478",
    "stun.hoiio.com:3478",
    "stun.hosteurope.de:3478",
    "stun.infra.net:3478",
    "stun.internetcalls.com:3478",
    "stun.intervoip.com:3478",
    "stun.ipfire.org:3478",
    "stun.ippi.fr:3478",
    "stun.ipshka.com:3478",
    "stun.it1.hr:3478",
    "stun.ivao.aero:3478",
    "stun.jumblo.com:3478",
    "stun.justvoip.com:3478",
    "stun.linphone.org:3478",
    "stun.liveo.fr:3478",
    "stun.lowratevoip.com:3478",
    "stun.lundimatin.fr:3478",
    "stun.mit.de:3478",
    "stun.miwifi.com:3478",
    "stun.modulus.gr:3478",
    "stun.myvoiptraffic.com:3478",
    "stun.netappel.com:3478",
    "stun.netgsm.com.tr:3478",
    "stun.nfon.net:3478",
    "stun.nonoh.net:3478",
    "stun.nottingham.ac.uk:3478",
    "stun.ooma.com:3478",
    "stun.ozekiphone.com:3478",
    "stun.pjsip.org:3478",
    "stun.poivy.com:3478",
    "stun.powervoip.com:3478",
    "stun.ppdi.com:3478",
    "stun.qq.com:3478",
    "stun.rackco.com:3478",
    "stun.rockenstein.de:3478",
    "stun.rolmail.net:3478",
    "stun.rynga.com:3478",
    "stun.schlund.de:3478",
    "stun.sigmavoip.com:3478",
    "stun.sip.us:3478",
    "stun.sipdiscount.com:3478",
    "stun.sipgate.net:10000",
    "stun.sipgate.net:3478",
    "stun.siplogin.de:3478",
    "stun.sipnet.net:3478",
    "stun.sipnet.ru:3478",
    "stun.sippeer.dk:3478",
    "stun.siptraffic.com:3478",
    "stun.sma.de:3478",
    "stun.smartvoip.com:3478",
    "stun.smsdiscount.com:3478",
    "stun.solcon.nl:3478",
    "stun.solnet.ch:3478",
    "stun.sonetel.com:3478",
    "stun.sonetel.net:3478",
    "stun.sovtest.ru:3478",
    "stun.srce.hr:3478",
    "stun.stunprotocol.org:3478",
    "stun.t-online.de:3478",
    "stun.tel.lu:3478",
    "stun.telbo.com:3478",
    "stun.tng.de:3478",
    "stun.twt.it:3478",
    "stun.uls.co.za:3478",
    "stun.unseen.is:3478",
    "stun.usfamily.net:3478",
    "stun.viva.gr:3478",
    "stun.vivox.com:3478",
    "stun.vo.lu:3478",
    "stun.voicetrading.com:3478",
    "stun.voip.aebc.com:3478",
    "stun.voip.blackberry.com:3478",
    "stun.voip.eutelia.it:3478",
    "stun.voipblast.com:3478",
    "stun.voipbuster.com:3478",
    "stun.voipbusterpro.com:3478",
    "stun.voipcheap.co.uk:3478",
    "stun.voipcheap.com:3478",
    "stun.voipgain.com:3478",
    "stun.voipgate.com:3478",
    "stun.voipinfocenter.com:3478",
    "stun.voipplanet.nl:3478",
    "stun.voippro.com:3478",
    "stun.voipraider.com:3478",
    "stun.voipstunt.com:3478",
    "stun.voipwise.com:3478",
    "stun.voipzoom.com:3478",
    "stun.voys.nl:3478",
    "stun.voztele.com:3478",
    "stun.webcalldirect.com:3478",
    "stun.wifirst.net:3478",
    "stun.xtratelecom.es:3478",
    "stun.zadarma.com:3478",
    "stun1.faktortel.com.au:3478",
    "stun1.l.google.com:19302",
    "stun2.l.google.com:19302",
    "stun3.l.google.com:19302",
    "stun4.l.google.com:19302",
    "stun.nextcloud.com:443",
    "relay.webwormhole.io:3478",
];

use std::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};
use stunclient::StunClient;

fn get_reflexive_transport_addr(stun_server_addr: &str, localport: u16) -> SocketAddr {
    let local_addr = SocketAddr::new("0.0.0.0".parse().unwrap(), localport);

    let stun_addr = stun_server_addr
        .to_socket_addrs()
        .unwrap()
        .filter(|x| x.is_ipv4())
        .next()
        .unwrap();

    let udp = UdpSocket::bind(local_addr).unwrap();
    let client = StunClient::new(stun_addr);

    return client.query_external_address(&udp).unwrap();
}

fn listen(stun_server_addr: &str, localport: u16) {
    let local_addr = SocketAddr::new("0.0.0.0".parse().unwrap(), localport);

    let stun_addr = stun_server_addr
        .to_socket_addrs()
        .unwrap()
        .filter(|x| x.is_ipv4())
        .next()
        .unwrap();

    let udp = UdpSocket::bind(local_addr).unwrap();

    let client = StunClient::new(stun_addr);
    let myaddr = client.query_external_address(&udp).unwrap();
    println!("listening on {}...", myaddr);
    let mut buf = [0; 10];
    let (amt, src) = udp.recv_from(&mut buf).unwrap();
    println!("packet received");
}

fn main() {
    let param = std::env::args().skip(1).next();
    let port = 19010;

    match param {
        Some(p) if p == "print" => {
            let myaddr = get_reflexive_transport_addr(STUN_SERVERS[0], port);
            println!("My addr was mapped from 0.0.0.0:{} to {}", port, myaddr);
        },
        Some(p) => {
            let local_addr = SocketAddr::new("0.0.0.0".parse().unwrap(), port);
            let socket = UdpSocket::bind(local_addr).expect("couldn't bind to address");
            socket.send_to(&[0; 10], p).expect("couldn't send data");
        },
        None => {
            listen(STUN_SERVERS[0], port);
        }
    }
}
