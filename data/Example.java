public class Example {
  private int sum;
  final long l = 32L;
  float f = 1.6f;
  public int example(int a, int b) {
    int c = a + b;
    sum = c;
    return c;
  }
  protected String exampleStr() {
    return String.format("%f", 3.1 + f);
  }
}
