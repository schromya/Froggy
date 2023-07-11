#include <iostream>
#include <cstdlib>
#include <string>
#include <time.h>

class frog {
    public:
        frog(std::string name, float weight);
        frog(std::string name);
        frog();
        void hop();
        void ribbit();
        //float* data();
        std::string name;
    private:
        int hop_num();
        std::string ribbit_sound();
        float weight, size;
};

frog::frog() {
    this->name = "Albert";
    this->weight = 1.0;
}

frog::frog(std::string name) {
    this-> name = name;
    this->weight = 1.0;
}

frog::frog(std::string name, float weight) {
    this-> name = name;
    this->weight = weight;
}

void frog::hop() {
    std::cout << name << " hopped " << hop_num() << " time(s)!" << '\n';
}

int frog::hop_num() {
    int out = rand() % 9 + 2;
    return out;
}

void frog::ribbit() {
    std::cout << name << " goes " << ribbit_sound() << '\n';
}

std::string frog::ribbit_sound() {
    int rnd = rand() % 4;
    switch (rnd)
    {
    case 0:
        return "Ribbit!";
        break;
    case 1:
        return "RibBit ribBit";
        break;
    case 2:
        return "ribBIT!";
        break;
    case 3:
        return "RiBbIt";
        break;
    
    default:
        return "<awkward silence>";
        break;
    }
}

// Driver program

int main() {

    srand (time(NULL));
    std::string name;
    std::cout << "Enter your frog's name: ";
    getline(std::cin, name);
    float weight;
    /*std::cout << "Enter " << name << "'s weight: ";
    std::cin >> weight;*/
    frog myFrog(name, weight);
    myFrog.hop();
    myFrog.ribbit();

    return 0;
}
