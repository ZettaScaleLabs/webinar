#include <iostream>
#include <zenoh.hxx>
#include "Image.pb.h"

std::ostream& operator<<(std::ostream& os, const Image& img) {
    if (img.format() == "ascii") {
        os << img.img() << std::endl;
    }
    else {
        os << "Image is not printable on console" << std::endl;
    }
    return os;
}

void render_image(const zenoh::Sample& s) {
    Image img;
    img.ParseFromArray(s.payload.start, s.payload.len);
    std::cout << img << std::endl;
}

int main() {
    zenoh::Config c;
    auto z = zenoh::expect<zenoh::Session>(
            zenoh::open(std::move(c)));
    auto s = zenoh::expect<zenoh::Subscriber>(
            z.declare_subscriber("demo/bulletin/ascii-image", render_image));

    std::cout << "Press Enter to exit." << std::endl;
    std::cin.get();
    return 0;
}
