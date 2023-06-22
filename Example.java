public class Example {
    private int intValue;
    public Example (int intValue) {
        this.intValue = intValue;
    }
    public static void main (String args) {
        Example example = new Example(10);
        System.out.println(
            "Hello, your intValue is "
            + example.intValue
        );
    }

    public void yesBeforeNoInside () {  }
    public void yesBeforeYesInside ( ) {  }
    public void noBeforeYesInside( ) {  }
    public void noBeforeNoInside() {  }
    public void noSpaceWhatsoever(){};

    public int intValue() {
        return this.intValue;
    }

    public String propertyAfterFunctions;
}