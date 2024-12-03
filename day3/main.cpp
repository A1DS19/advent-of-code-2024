#include <fstream>
#include <iostream>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

struct Match {
  std::string text;
  size_t position;
};

std::vector<Match> find_all_matches(const std::string &input,
                                    const std::regex &pattern) {
  std::vector<Match> matches;
  auto words_begin = std::sregex_iterator(input.begin(), input.end(), pattern);
  auto words_end = std::sregex_iterator();

  for (auto i = words_begin; i != words_end; ++i) {
    matches.push_back({(*i).str(), static_cast<size_t>((*i).position())});
  }
  return matches;
}

std::vector<std::string>
extract_enabled_multiplications(const std::string &input) {
  std::vector<std::string> results;
  std::regex mul_pattern(R"(mul\(\d+,\d+\))");
  std::regex do_pattern(R"(do\(\))");
  std::regex dont_pattern(R"(don't\(\))");

  auto muls = find_all_matches(input, mul_pattern);
  auto dos = find_all_matches(input, do_pattern);
  auto donts = find_all_matches(input, dont_pattern);

  bool enabled = true;
  size_t last_control = 0;

  for (const auto &mul : muls) {
    for (const auto &do_match : dos) {
      if (do_match.position < mul.position &&
          do_match.position > last_control) {
        enabled = true;
        last_control = do_match.position;
      }
    }
    for (const auto &dont_match : donts) {
      if (dont_match.position < mul.position &&
          dont_match.position > last_control) {
        enabled = false;
        last_control = dont_match.position;
      }
    }

    if (enabled) {
      results.push_back(mul.text);
    }
  }

  return results;
}

std::vector<std::string> multiply(const std::vector<std::string> &input) {
  std::vector<std::string> results;
  results.reserve(input.size());

  for (const auto &expression : input) {
    std::regex pattern(R"(\d+)");
    auto words_begin =
        std::sregex_iterator(expression.begin(), expression.end(), pattern);
    auto words_end = std::sregex_iterator();

    std::vector<int> numbers;
    for (std::sregex_iterator i = words_begin; i != words_end; ++i) {
      numbers.push_back(std::stoi((*i).str()));
    }

    results.push_back(std::to_string(numbers[0] * numbers[1]));
  }

  return results;
}

long int add_multiplied(const std::vector<std::string> &input) {
  long int result = 0;
  try {
    result = std::accumulate(input.begin(), input.end(), 0L,
                             [](long int sum, const std::string &num) {
                               return sum + std::stol(num);
                             });
  } catch (const std::overflow_error &e) {
    throw std::runtime_error("Overflow occurred during addition");
  } catch (const std::invalid_argument &e) {
    throw std::runtime_error("Invalid number format");
  }

  return result;
}

int main() {
  try {
    std::ifstream file("input.txt");
    if (!file) {
      throw std::runtime_error("Could not open file");
    }

    std::string content((std::istreambuf_iterator<char>(file)),
                        std::istreambuf_iterator<char>());

    auto results = extract_enabled_multiplications(content);
    auto multiplied = multiply(results);
    auto sum = add_multiplied(multiplied);

    std::cout << "Result: " << sum << '\n';

    return 0;
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << '\n';
    return 1;
  }
}