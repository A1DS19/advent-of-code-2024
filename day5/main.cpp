#include <algorithm>
#include <fstream>
#include <iostream>
#include <queue>
#include <set>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

class PrintQueue {
private:
  std::unordered_map<int, std::vector<int>> graph;

  std::vector<int> topologicalSort(const std::vector<int> &nums) {
    std::unordered_map<int, std::vector<int>> localGraph;
    std::unordered_map<int, int> inDegree;
    std::set<int> numbers(nums.begin(), nums.end());

    // Build local graph with only relevant nodes
    for (const auto &pair : graph) {
      if (numbers.count(pair.first)) {
        for (int to : pair.second) {
          if (numbers.count(to)) {
            localGraph[pair.first].push_back(to);
            inDegree[to]++;
            if (inDegree.find(pair.first) == inDegree.end()) {
              inDegree[pair.first] = 0;
            }
          }
        }
      }
    }

    // Topological sort
    std::vector<int> result;
    std::queue<int> q;

    // Find nodes with no incoming edges
    for (int num : nums) {
      if (inDegree[num] == 0) {
        q.push(num);
      }
    }

    while (!q.empty()) {
      int current = q.front();
      q.pop();
      result.push_back(current);

      for (int next : localGraph[current]) {
        inDegree[next]--;
        if (inDegree[next] == 0) {
          q.push(next);
        }
      }
    }

    return result;
  }

public:
  void addRule(int from, int to) { graph[from].push_back(to); }

  bool isValidSequence(const std::vector<int> &seq) {
    std::set<int> numbers(seq.begin(), seq.end());

    for (size_t i = 0; i < seq.size(); i++) {
      for (size_t j = i + 1; j < seq.size(); j++) {
        int first = seq[i];
        int second = seq[j];
        if (hasPath(second, first, numbers)) {
          return false;
        }
      }
    }
    return true;
  }

  bool hasPath(int start, int end, const std::set<int> &validNodes) {
    if (start == end)
      return true;

    std::queue<int> q;
    std::set<int> visited;
    q.push(start);
    visited.insert(start);

    while (!q.empty()) {
      int current = q.front();
      q.pop();

      for (int next : graph[current]) {
        if (!validNodes.count(next))
          continue;
        if (next == end)
          return true;
        if (visited.count(next))
          continue;

        visited.insert(next);
        q.push(next);
      }
    }
    return false;
  }

  std::vector<int> getCorrectOrder(const std::vector<int> &seq) {
    return topologicalSort(seq);
  }

  int getMiddleNumber(const std::vector<int> &seq) {
    return seq[seq.size() / 2];
  }
};

int main() {
  PrintQueue queue;
  std::string line;
  bool parsingRules = true;
  int sum = 0;
  std::vector<std::vector<int>> invalidSequences;

  std::ifstream inputFile("input.txt");
  if (!inputFile.is_open()) {
    std::cerr << "Error: Could not open input.txt" << std::endl;
    return 1;
  }

  while (std::getline(inputFile, line)) {
    if (line.empty()) {
      parsingRules = false;
      continue;
    }

    if (parsingRules) {
      size_t pos = line.find('|');
      int from = std::stoi(line.substr(0, pos));
      int to = std::stoi(line.substr(pos + 1));
      queue.addRule(from, to);
    } else {
      std::stringstream ss(line);
      std::vector<int> sequence;
      std::string number;
      while (std::getline(ss, number, ',')) {
        sequence.push_back(std::stoi(number));
      }

      if (!queue.isValidSequence(sequence)) {
        invalidSequences.push_back(sequence);
      }
    }
  }

  // Process invalid sequences
  for (const auto &seq : invalidSequences) {
    std::vector<int> correctedSeq = queue.getCorrectOrder(seq);
    sum += queue.getMiddleNumber(correctedSeq);
  }

  inputFile.close();
  std::cout << "Sum of middle numbers after correction: " << sum << std::endl;
  return 0;
}