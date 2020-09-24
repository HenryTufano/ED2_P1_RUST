use std::io;
fn main() {
    
    let mut choice:usize = 0;
    while choice!=5{
        println!("=========================================================\nATIVIDADE AVALIATIVA - P1\nLINGUAGENS E TÉCNICAS DE PROGRAMAÇÃO\n[ 1 ] - VERIFICAR PARIDADE\n[ 2 ] - VERIFICAR POLARIDADE\n[ 3 ] - CRÉDITO ESPECIAL\n[ 4 ] - TRIANGULO\n[ 5 ] - SAIR DO PROGRAMA\n=========================================================\nDigite sua Opção: ");
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line");
        let choice:usize = x
                        .trim()
                        .parse()
                        .expect("Opção incorreta");
            if choice ==1{
                println!("Digite um numero maior que 0: ");
                let mut n1 = String::new();
                io::stdin()
                    .read_line(&mut n1)
                    .expect("Failed to read line");
                let mut n1u:usize = n1
                                .trim()
                                .parse()
                                .expect("Opção incorreta");
                verifica_paridade(n1u);
                
            }if choice ==2{
                
                println!("Digite um numero : ");
                let mut n1 = String::new();
                io::stdin()
                    .read_line(&mut n1)
                    .expect("Failed to read line");
                let mut n1u:isize = n1
                                .trim()
                                .parse()
                                .expect("Opção incorreta");
                verifica_polaridade(n1u);
                

            }if choice ==3{
                println!("Digite o valor do saldo medio: ");
                let mut sm = String::new();
                io::stdin()
                    .read_line(&mut sm)
                    .expect("Failed to read line");
                let mut smu:f32 = sm
                                .trim()
                                .parse()
                                .expect("Opção incorreta");
                verifica_credito(smu);

            }
            if choice ==4{
                println!("Digite o valor do primeiro triangulo: ");
                let mut tx = String::new();
                io::stdin()
                    .read_line(&mut tx)
                    .expect("Failed to read line");
                let mut txu:usize = tx
                                .trim()
                                .parse()
                                .expect("Opção incorreta");

                println!("Digite o valor do segundo triangulo: ");
                let mut ty = String::new();
                io::stdin()
                    .read_line(&mut ty)
                    .expect("Failed to read line");
                let mut tyu:usize = ty
                                .trim()
                                .parse()
                                .expect("Opção incorreta");

                println!("Digite o valor do terceiro triangulo: ");
                let mut tz = String::new();
                io::stdin()
                    .read_line(&mut tz)
                    .expect("Failed to read line");
                let mut tzu:usize = tz
                                .trim()
                                .parse()
                                .expect("Opção incorreta");
                verifica_triangulo(txu,tyu,tzu);
            }if choice ==5{
                break;
            }
     
    
    }
    

}

fn verifica_paridade(n1u:usize){
    if n1u%2==0{
        println!("numero par");
       
    }else{
        println!("numero impar");
    }
}

fn verifica_polaridade (n1u:isize){
    if n1u>=0{
        println!("O número informado é POSITIVO");        
       
    }else if n1u<0{
        println!("O número informado é NEGATIVO");
    }
}

fn verifica_credito(smu:f32){
    let mut ce = String::new();
    io::stdin()
        .read_line(&mut ce)
        .expect("Failed to read line");
    let mut ceu:f32 = ce
                    .trim()
                    .parse()
                    .expect("Opção incorreta");

    if smu<201.0{
        println!("Nenhum credito disponivel");
    }else if smu>=201.0 && smu<=400.0{
        let ceu=smu*0.2;
        println!("Caro cliente, como o seu saldo médio foi de {}, o valor de seu crédito especial será de {}",smu,ceu);
    }else if smu>=401.0 && smu<=600.0{
        let ceu=smu*0.3;
        println!("Caro cliente, como o seu saldo médio foi de {}, o valor de seu crédito especial será de {}",smu,ceu);
    }else if smu>600.0{
        let ceu=smu*0.4;
        println!("Caro cliente, como o seu saldo médio foi de {}, o valor de seu crédito especial será de {}",smu,ceu);        
    }
}

fn verifica_triangulo(txu:usize,tyu:usize,tzu:usize){
    if txu<=tyu+tzu && tyu<=tzu+txu && tzu<=txu+tyu &&txu>0&&tyu>0&&tzu>0{
        if txu==tyu &&txu==tzu&&tyu==tzu{
            println!("O triangulo é Equilatero")
        }else if (txu==tyu &&txu!=tzu)|(tyu==tzu&&tyu!=txu)|(txu==tzu&&txu!=tyu){
            println!("O triangulo é isosceles");
        }else if txu!=tyu&&txu!=tzu&&tyu!=tzu{
            println!("O triangulo é escaleno");
        }

    }else{
        println!(" Triangulo inexistente")
    }

}