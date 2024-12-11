#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
using namespace std;

string multiplyBy2024(const string &num) {
  int mul = 2024;
  int carry = 0;
  vector<int> result;
  result.reserve(num.size() + 5);

  for (int i = (int)num.size() - 1; i >= 0; i--) {
    int digit = num[i] - '0';
    long long prod = (long long)digit * mul + carry;
    carry = (int)(prod / 10);
    result.push_back((int)(prod % 10));
  }
  while (carry > 0) {
    result.push_back(carry % 10);
    carry /= 10;
  }
  reverse(result.begin(), result.end());
  string result_str;
  for (int digit : result) {
    result_str.push_back(digit + '0');
  }
  return result_str;
}

string stripLeadingZeros(const string &num) {
  size_t non_zero = num.find_first_not_of('0');
  if (non_zero == string::npos) {
    return "0";
  }
  return num.substr(non_zero);
}

void simulate_blinks(int blinks, const string &input_file) {
  vector<string> stones;
  ifstream file(input_file);
  if (!file.is_open()) {
    cerr << "Failed to open " << input_file << "\n";
    exit(EXIT_FAILURE);
  }

  string line;
  while (getline(file, line)) {
    istringstream iss(line);
    string num;
    while (iss >> num) {
      stones.push_back(num);
    }
  }
  file.close();

  for (int i = 0; i < blinks; i++) {
    vector<string> newStones;
    for (const auto &stone : stones) {
      if (stone == "0") {
        newStones.push_back("1");
      } else {
        int len = stone.size();
        if (len % 2 == 0) {
          int mid = len / 2;
          string leftHalf = stone.substr(0, mid);
          string rightHalf = stone.substr(mid);
          leftHalf = stripLeadingZeros(leftHalf);
          rightHalf = stripLeadingZeros(rightHalf);

          newStones.push_back(leftHalf);
          newStones.push_back(rightHalf);
        } else {
          newStones.push_back(multiplyBy2024(stone));
        }
      }
    }
    stones.swap(newStones);
  }

  cout << stones.size() << "\n";
}

int main() {
  simulate_blinks(75, "input.txt");

  return 0;
}