#include <iostream>
#include <cstdlib>
#include <string>
#include <time.h>
#include <chrono>
#include <thread>

class frog {
    public:
        frog(std::string name, float weight);
        frog(std::string name);
        frog();
        void frog_life();
        void hop();
        void ribbit();
        void test_weight();
        std::string calculate_weight();
        void munchies();
        std::string name;
        bool is_chilling = 1;
    private:
        int hop_num();
        std::string ribbit_sound();
        float weight, size;
        bool is_superfrog;
};

frog::frog() {
    this->name = "Froog";
    this->weight = 1.0;
    is_superfrog = 0;
}

frog::frog(std::string name) {
    this-> name = name;
    this->weight = 1.0;
}

frog::frog(std::string name, float weight) {
    this-> name = name;
    this->weight = weight;
}

void frog::frog_life() {

    std::cout << std::endl << name << " is chilling, what do you want to do?" << '\n';
    std::cout << "| 0. Quit | 1. Poke | 2. Prod | 3. Put on scale | 4. Feed | Choice: ";
    int input;
    std::cin >> input;
    while (input < 0 || input > 4) {
        std::cout << "Invalid input. Try again" << '\n';
        std::cin >> input;
    }
    std::cout << std::endl;
    switch (input) {
        case 0: {is_chilling = 0;} break;
        case 1: {ribbit();} break;
        case 2: {hop();} break;
        case 3: {test_weight();} break;
        case 4: {munchies();} break;
    }
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

void frog::test_weight() {
    std::cout << "Testing your froggy's weight";
    for (int i = 0; i < 3; i++) {
        std::cout.flush();
        std::this_thread::sleep_for(std::chrono::milliseconds(750));
        std::cout << " .";
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(750));
    std::cout << std::endl;
    std::cout << "Ding Ding Ding: " << calculate_weight() << std::endl;
}
std::string frog::calculate_weight() {
    if (weight < 0.0) return "Need more frog, lest ghost.";
    else if (weight < 2.0) return "Froggy needs more bugs and munchies.";
    else if (weight < 4.0) return "Froggy is big boy.";
    return "What does frog eat? Humans?";

}

void frog::munchies() {
    int input;
    std::cout << name << " is hungry. What should frog eat?" << '\n';
    std::cout << "| 1. Flies | 2. Slippery Fiskies | 3. Humans | 4. Strange green stuff | Choice: ";
    std::cin >> input;
    while (input < 1 || input > 4) {
        std::cout << "Input invalid. Try again" << '\n';
        std::cin >> input;
    }
    std::cout << std::endl;
    switch (input) {
        case 1: {
            std::cout << "*  " << "Froggy consumed lots of flies. Looking fatter now" << '\n';
            weight += 2.0;
        }
        break;
        case 2: {
            std::cout << "*  " << "Froggy downed some slippery fiskies" << '\n';
            weight += 3.0;
        }
        break;
        case 3: {
            std::cout << "*  " << "Froggy chomped humans. Ouch" << '\n';
            weight += 5.0;
        }
        break;
        case 4: {
            std::cout << "*  " << "Uh oh" << '\n';
            weight = 10000;
            is_superfrog = 1;
        }
        break;
    }
    std::cout << ">> " << name << " had a case of the munchies, but that's all better now!\n" << ">> " << ribbit_sound() << '\n';

}

// Driver program

int main() {

    srand (time(NULL));
    std::string name;
    std::cout << "Enter your frog's name: ";
    getline(std::cin, name);
    float weight;
    std::cout << "Enter " << name << "'s weight: ";
    std::cin >> weight;
    frog myFrog(name, weight);
    do {
        myFrog.frog_life();
    } while (myFrog.is_chilling);

    return 0;
}
