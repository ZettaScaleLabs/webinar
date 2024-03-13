#include <iostream>
#include <zenoh.hxx>
#include "Image.pb.h"

extern const std::string light_blue_dragon;
extern const std::string blue_dragon;


int main() {
    zenoh::Config c;
    auto z = zenoh::expect<zenoh::Session>(
            zenoh::open(std::move(c)));
    auto p = zenoh::expect(z.declare_publisher("demo/bulletin/ascii-image"));
    Image lbd;
    lbd.set_width(80);
    lbd.set_height(34);
    lbd.set_format("ascii");
    lbd.set_img(light_blue_dragon);
    std::string slbd;
    lbd.SerializeToString(&slbd);

    Image bd;
    bd.set_width(80);
    bd.set_height(34);
    bd.set_format("ascii");
    bd.set_img(blue_dragon);
    bd.set_img(blue_dragon);

    std::string sbd;
    bd.SerializeToString(&sbd);


    int s = 1;
    while (s != 0) {
        std::cout << "Select the image to send:\n";
        std::cout << " (1) -- Zenoh Light Logo\n";
        std::cout << " (2) -- Zenoh Logo\n";
        std::cout << " (0) -- Exit" << std::endl;
        std::cin >> s;

        auto p_opts = zenoh::PutOptions();
        p_opts
            .set_encoding(zenoh::EncodingPrefix::Z_ENCODING_PREFIX_APP_CUSTOM)
            .set_congestion_control(zenoh::CongestionControl ::Z_CONGESTION_CONTROL_DROP)
            .set_priority(zenoh::Priority::Z_PRIORITY_INTERACTIVE_HIGH);


        switch (s) {
            case 1:
                // Just to showcase producing data w/o a publisher

                z.put("demo/bulletin/ascii-image", slbd, p_opts);
                break;
            case 2:
                // Just to showcase publishing with a publisher
                p.put(sbd);
                break;
            default:
                break;
        }
    }
    return 0;
}
