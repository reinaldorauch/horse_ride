//
// Adaptado do livro Projeto de Algoritmos de Nivio Ziviani
//
// Disciplina: Análise de Algoritmos - DEINFO/UEPG
// Proj. Jaime Cohen

#include <cstdlib>
#include <iostream>

using namespace std;

class PasseioCavalo {
private:
  int n;
  int **t;
  int a[8] = {2, 1, -1, -2, -2, -1, 1, 2};
  int b[8] = {1, 2, 2, 1, -1, -2, -2, -1};

public:
  PasseioCavalo(int n, int x0, int y0);
  bool tenta(int i, int x, int y);
  void imprimePasseio() const;
  ~PasseioCavalo();
};

// construtor da classe
PasseioCavalo::PasseioCavalo(int n, int x0, int y0) {
  this->n = n;
  this->t = new int *[n];
  for (int i = 0; i < n; i++)
    this->t[i] = new int[n];
  for (int i = 0; i < n; i++)
    for (int j = 0; j < n; j++)
      t[i][j] = -1;
  t[x0][y0] = 0;
}

// destrutor da classe
PasseioCavalo::~PasseioCavalo() {
  for (int i = 0; i < n; i++)
    delete[] this->t[i];
  delete[] this->t;
}

// contador para o núm. de chamadas recursivas
int nodos = 0;

// método principal: backtracking recursivo
bool PasseioCavalo::tenta(int i, int x, int y) {
  int u, v, k;
  bool q;
  ++nodos;
  for (int k = 0; k <= 7; k++) {
    u = x + a[k];
    v = y + b[k];
    if (u >= 0 && u < n && v >= 0 && v < n && t[u][v] == -1) {
      t[u][v] = i;
      if (i == (n * n - 1)) // completou o percurso
        return true;
      if (tenta(i + 1, u, v))
        return true;
      else // desfaz o movimento (backtrack)
        t[u][v] = -1;
    }
  }
  return false;
}

void PasseioCavalo::imprimePasseio() const {
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < n; j++)
      cout << "\t" << this->t[i][j];
    cout << endl;
  }
  cout << endl;
}

int main(int argc, char *argv[]) {
  int n;
  if (argc == 2)
    n = atoi(argv[1]);
  else
    n = 8;

  int x0 = 0, y0 = 0;

  PasseioCavalo passeioCavalo(n, x0, y0);

  // chamada inicial: tenta(1, x_0, y_0)
  bool q = passeioCavalo.tenta(1, x0, y0);

  if (q)
    passeioCavalo.imprimePasseio();
  else
    cout << "Sem solucao" << endl;

  cout << "Movimentos feitos: " << nodos << endl;
  return 0;
}
