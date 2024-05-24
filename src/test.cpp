#include <iostream>

int main() {
    int cost, coins = 0;   
    std::cin >> cost;
    coins += cost / 25;
    cost %= 25;
    std::cout << "25: cost=" << cost << ", coins=" << coins << "\n"; 
    coins += cost / 10;
    cost %= 10;
    std::cout << "10: cost=" << cost << ", coins=" << coins << "\n"; 
    coins += cost / 5;
    cost %= 5;
    std::cout << "5: cost=" << cost << ", coins=" << coins << "\n"; 
    coins += cost;
    std::cout << "final: cost=" << cost << ", coins=" << coins << "\n"; 

    std::cout << coins << "\n";
}