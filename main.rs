fn main()
{
    let mut a = [10, 20, 30, 40, 50];

    let mut length = a.len();
    println!("{length}");

    for element in a {
        println!("the value is: {element}");
    }
    for value in (0..3)
    {
        let mut temp = a[value];
        a[value] = a[length-1];
        a[length-1] = temp;
        length = length-1;
       
    }
    for number in (0..5)
    {
        println!("The rotated array is {}" , a[number]);
    }


    
}
