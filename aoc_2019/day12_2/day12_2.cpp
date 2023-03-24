#include <iostream>
#include <cstdlib>
#include <algorithm>
#include <numeric>
#include "Utilities.h"

using std::cout;
using std::cin;
using std::cerr;
using std::endl;

struct Position {
    int x;
    int y;
    int z;
    bool operator==(Position pos) {
        return (this->x == pos.x) && (this->y == pos.y) && (this->z == pos.z);
    }
};

struct Velocity {
    int x;
    int y;
    int z;

    bool operator==(Velocity vel) {
        return (this->x == vel.x) && (this->y == vel.y) && (this->z == vel.z);
    }
};

class Moon {
    Position posn;
    Velocity vel;
    double potEnergy;
    double kinEnergy;
    double totEnergy;

    Moon() = delete;
    public:
        Moon(int _x, int _y, int _z) {
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

        friend std::ostream & operator<<(std::ostream & os, const Moon & mn) {
            os << "(" << mn.posn.x << ", " << mn.posn.y << ", " << mn.posn.z << ") "
               << "-> (" << mn.vel.x << ", " << mn.vel.y << ", " << mn.vel.z << ")";
            return os;
        }
};

int main(int argc, const char * argv[]) {
    Moon m1(-5, 6, -11);
    Moon m2(-8, -4, -2);
    Moon m3(1, 16, 4);
    Moon m4(11, 11, -4);

    Moon m1_ini = m1;
    Moon m2_ini = m2;
    Moon m3_ini = m3;
    Moon m4_ini = m4;

    bool matchFound = false;
    long steps = 1;

    while (!matchFound) {
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

        m1.applyVelocity();
        m2.applyVelocity();
        m3.applyVelocity();
        m4.applyVelocity();
        steps++;

        Position m1_pos = m1.getPosition();
        Position m1_ini_pos = m1_ini.getPosition();

        Position m2_pos = m2.getPosition();
        Position m2_ini_pos = m2_ini.getPosition();

        Position m3_pos = m3.getPosition();
        Position m3_ini_pos = m3_ini.getPosition();

        Position m4_pos = m4.getPosition();
        Position m4_ini_pos = m4_ini.getPosition();

        if ( (m1_pos == m1_ini_pos) && (m2_pos == m2_ini_pos) && (m3_pos == m3_ini_pos) && (m4_pos == m4_ini_pos)) {
            matchFound = true;
        }
    }

    cout << "Exact match found at step " << steps << endl;

    return EXIT_SUCCESS;
}

