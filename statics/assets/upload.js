// const server = '127.0.0.1:8080';

async function post_req(file, id) {
    let req_body = {
        bytes: await file.slice(0, 50).text(),
        id: id,
    };
    console.log(req_body);
    
    let request = await fetch(`/u`, {
        method: 'POST',
        headers: { 
            'Access-Control-Allow-Headers': '*',
            'Access-Control-Allow-Origin' : '*',
        },
        body: JSON.stringify(req_body),
    });
    
    let reader = await request.body.getReader();
    let response_val = await reader.read();
    console.log(response_val);
    return response_val;
}

