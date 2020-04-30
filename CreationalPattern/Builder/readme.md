# Builder 
빌더 패턴의 경우 정말 여러가지의 방법들이 있는 것 같다.
같은 빌더 패턴인데, 개념이 댜른 부분들이 여러가지 있었던 것 같다.

 - Javabeens Pattern
 - Builder Pattern (Effective Java 기준)
 - Builder Pattern (GoF 기준)

아래에는 제가 이해한 기준으로 우선 개념 설명을 진행하며, GoF 기준으로 진행되는 공부이기 때문에 코드는 GoF기준으로 진행했습니다. 하지만, 개인적으로 Effective Java기준의 Builder 패턴이 실사용에서 좋다고 판단됩니다.

## Javabeans
Javabeens 패턴은 생성자에 인자가 많을 때 가독성과 사용 오류를 막아주는 패턴입니다. <br>
인자를 그 때, 그 때 setter 함수로 지정을 해주며, 인스턴스를 반환해줍니다.

## Builder Pattern (Effective Java 기준)
Effective Java 기준의 Builder 패턴같은 경우는 Javabeens에 상위 호환 패턴인 것 같습니다. Javabeens 패턴같은 경우 인자를 지정해줄 때 아에 인스턴스를 반환해주기 때문에 `객체의 일관성` 이 깨지게됩니다. <br>
하지만, 해당 패턴을 사용하게되면, Builder에 대한 인스턴스를 반환하게되고, Static Method를 이요하여 빌드하여야 인스턴스를 반환하게 됩니다.

## Builder Pattern (GoF 기준)
GoF에서의 Builder패턴은 `객체를 생성하는 방법` 과 `객체를 표현하는 방법` 을 분리한 디자인 패턴이라고 보시면 됩니다. 즉, 빌더와 빌더를 사용하는 `Director` 가 존재하며, Director를 통해 build가 이뤄지게 됩니다. <br><br>

자세한 내용은 코드를 참조하시면 될 것 같으며, 제가 참조한 글은 아래 블로그입니다. <br><br>
[기계인간 John Grib](https://johngrib.github.io/wiki/builder-pattern/)