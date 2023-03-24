#include <iostream>
#include <cstdlib>
#include <vector>
#include <map>
#include <algorithm>
#include <tuple>
#include <memory>
#include <numeric>
#include <future>
#include "Utilities.h"

using std::string;
using std::cout;
using std::cin;
using std::cerr;
using std::endl;
using std::vector;
using std::map;
using std::tuple;

struct Position {
    int x;
    int y;
    int z;
};

struct Velocity {
    int x;
    int y;
    int z;
};

class Moon {
    Position posn;
    Velocity vel;
    string name;
    double potEnergy;
    double kinEnergy;
    double totEnergy;

    Moon() = delete;
    public:
        Moon(string _name, int _x, int _y, int _z) {
            this->name = _name;
            this->posn.x = _x;
            this->posn.y = _y;
            this->posn.z = _z;

            this->vel.x = this->vel.y = this->vel.z = 0;
        }

        Position getPosition(void) const {
            return this->posn;
        }

        void applyGravity(const Moon & _ref) {
            Position refPosn = _ref.getPosition();

            if (this->posn.x < refPosn.x) {
                this->vel.x += 1;
            } else if (this->posn.x > refPosn.x) {
                this->vel.x -= 1;
            }

            if (this->posn.y < refPosn.y) {
                this->vel.y += 1;
            } else if (this->posn.y > refPosn.y) {
                this->vel.y -= 1;
            }

            if (this->posn.z < refPosn.z) {
                this->vel.z += 1;
            } else if (this->posn.z > refPosn.z) {
                this->vel.z -= 1;
            }
        }

        void applyVelocity(void) {
            this->posn.x += this->vel.x;
            this->posn.y += this->vel.y;
            this->posn.z += this->vel.z;
        }

        void calcEnergy() {
            this->potEnergy = std::abs(this->posn.x) + std::abs(this->posn.y) + std::abs(this->posn.z);
            this->kinEnergy = std::abs(this->vel.x) + std::abs(this->vel.y) + std::abs(this->vel.z);
            this->totEnergy = this->potEnergy * this->kinEnergy;

            /*** /
            cout << "PO energy -> " << this->potEnergy << endl;
            cout << "Kin energy -> " << this->kinEnergy << endl;
            cout << "Total energy -> " << this->totEnergy << endl;
            / ***/
        }
        void calcVelocityAndEnergy() {
            this->applyVelocity();
            this->calcEnergy();
        }

        double getTotalEnergy(void) const {
            return this->totEnergy;
        }

        friend std::ostream & operator<<(std::ostream & os, const Moon & mn) {
            os << "(" << mn.posn.x << ", " << mn.posn.y << ", " << mn.posn.z << ") "
               << "-> (" << mn.vel.x << ", " << mn.vel.y << ", " << mn.vel.z << ")";
            return os;
        }
};

int main(int argc, const char * argv[]) {
    Moon m1("Io", -5, 6, -11);
    Moon m2("Europa", -8, -4, -2);
    Moon m3("Ganymede", 1, 16, 4);
    Moon m4("Callisto", 11, 11, -4);

    for (int cnt = 1; cnt <= 1000; cnt++) {
        // Apply gravity for m1
        m1.applyGravity(m2);
        m1.applyGravity(m3);
        m1.applyGravity(m4);

        // Apply gravity for m2
        m2.applyGravity(m1);
        m2.applyGravity(m3);
        m2.applyGravity(m4);

        // Apply gravity for m3
        m3.applyGravity(m1);
        m3.applyGravity(m2);
        m3.applyGravity(m4);

        // Apply gravity for m4
        m4.applyGravity(m1);
        m4.applyGravity(m2);
        m4.applyGravity(m3);

        m1.calcVelocityAndEnergy();
        m2.calcVelocityAndEnergy();
        m3.calcVelocityAndEnergy();
        m4.calcVelocityAndEnergy();

        /*** /
        cout << m1 << endl;
        cout << m2 << endl;
        cout << m3 << endl;
        cout << m4 << endl;
        / ***/
    }

    double totEnergy = m1.getTotalEnergy() + m2.getTotalEnergy() + m3.getTotalEnergy() + m4.getTotalEnergy();
    cout << "Total energy after 10 steps is " << totEnergy << endl;

    return EXIT_SUCCESS;
}

