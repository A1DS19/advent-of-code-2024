#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
using namespace std;

inline void stripLeadingZerosInPlace(string &num) {
  // Find first non-zero character
  size_t pos = 0;
  while (pos < num.size() && num[pos] == '0') {
    pos++;
  }
  if (pos == num.size()) {
    // All zeros
    num = "0";
  } else if (pos > 0) {
    // Strip leading zeros
    num.erase(0, pos);
  }
}

string multiplyBy2024(const string &num) {
  // Multiply large number by 2024
  // 2024 = 2 * 1012, but let's just do a direct multiplication
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

  // Convert back to string
  string result_str;
  result_str.reserve(result.size());
  for (int digit : result) {
    result_str.push_back((char)('0' + digit));
  }
  return result_str;
}

void simulate_blinks(int blinks, const string &input_file) {
  vector<string> stones;
  {
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
  }

  // We know the number of stones can grow dramatically.
  // Try to avoid unnecessary copies by using reserve where possible.

  for (int i = 0; i < blinks; i++) {
    // Estimate the growth to reserve capacity
    // Worst case: every stone with an even number of digits splits into two.
    // Let's assume a worst-case doubling to at least reduce reallocations.
    vector<string> newStones;
    newStones.reserve(stones.size() * 2);

    for (auto &stone : stones) {
      if (stone == "0") {
        // Rule 1
        newStones.push_back("1");
      } else {
        int len = (int)stone.size();
        if (len % 2 == 0) {
          // Rule 2: even number of digits, split in half
          int mid = len / 2;
          string leftHalf = stone.substr(0, mid);
          string rightHalf = stone.substr(mid);

          stripLeadingZerosInPlace(leftHalf);
          stripLeadingZerosInPlace(rightHalf);

          newStones.push_back(move(leftHalf));
          newStones.push_back(move(rightHalf));
        } else {
          // Rule 3: multiply by 2024
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
