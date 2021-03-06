# 생성 패턴 (Creational Pattern)

> ### 생성 패턴은 인스턴스를 만드는 절차를 추상화하는 패턴입니다.
> 
> 해당 범주에 속하는 패턴은 객체를 생성 및 합성하는 방법이나 객체의 표현 방법과 시스템을 분리해 줍니다. 클래스 생성 패턴이 인스턴스로 만들 클래스를 다양하게 만들기 위한 용도로 상속을 사용한다면, 객체 생성 패턴은 인스턴스화 작업을 다른 객체에게 떠넘길 수도 있습니다.

> ### 생성 패턴은 시스템이 상속보다는 복합 방법을 사요하는 쪽으로 진화되어 가면서 더 중요해지고 있습니다.
> 
> 고정된 행동 집합을 정의하는 것보다는, 더 복잡한 행동을 만드는데 필요한 구성요소가 될 수 있는 기본적인 행동 집합을 정의하는 쪽에 더 많은 관심과 노력이 들어가고 있습니다.
> **그러므로** 특정 행동을 수행하는 클래스를 만들려면 단순하게 하나의 클래스를 인스턴스화하는 일 이상의 품이 들어갑니다.

> ### 생성 패턴이 나오면 항상 따라오는 두 가지
> 1. 생성 패턴은 시스템이 어떤 구체 클래스를 사용하는지에 대한 정보를 캡슐화한다.
> 2. 생성 패턴은 이들 클래스의 인스턴스들이 어떻게 만들고 어떻게 서로 맞붙는지에 대한 부분을 완전히 가려준다.
> 
> 결론적으로, 생성 패턴을 이용하면 무엇이 생성되고, 누가 이것을 생성하며, 이것이 어떻게 생성되는지, 언제 생성할 것인지 결정하는 데 유연성을 확보할 수 있게 됩니다.