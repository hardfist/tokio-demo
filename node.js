async function long_task(name){
    console.log(name);
    let x = 0;
    for (i=0;i<1000000;i++){
        x = x + i*i; 
    }
    
}
async function t1(){
    for (let i=0;i<1000000;i++){
        await long_task('t1')
    }

}

async function t2(){
    for (let i=0;i<1000000;i++){
        await long_task('t2')
    }

}
async function main(){
    await Promise.all([t1(),t2()])
}
main();