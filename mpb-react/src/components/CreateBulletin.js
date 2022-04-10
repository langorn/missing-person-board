import { useState } from "react";

const CreateBulletin = ({ contract }) => {
    const [task, setTask] = useState({});
    const [loading, setLoading] = useState(false);

    const handleSubmit = async (event) => {
        event.preventDefault();
        setLoading(true);


        const createBulletin = await contract.new_bulletinpost({...task});
        setTask({img_url: "", location: "", description: "", contact:""});
        setLoading(false);
        console.log('my bulletin', createBulletin);
    };

    var handleChange = ( itemName, e) => {
        console.log('handlechange');
        let updateValue = {};
        updateValue[itemName] = e.target.value ;
        setTask(task => ({
            ...task,
            ...updateValue
        }));
    }

    return (
        <form onSubmit={handleSubmit}>
            <input
                type="text"
                placeholder="Image Url"
                value={task.img_url}
                onChange ={({target}) => setTask({...task, 'img_url': target.value})}
            />
            <input
                type="text"
                placeholder="Location"
                value={task.location}
                onChange ={({target}) =>  setTask({...task, 'location': target.value})}
            />
            <input
                type="text"
                placeholder="Description"
                value={task.description}
                onChange ={({target}) =>  setTask({...task, 'description': target.value})}
            />
            <input
                type="text"
                placeholder="Contact"
                value={task.contact}
                onChange ={({target}) =>  setTask({...task, 'contact': target.value})}
            />

            <button disabled={loading}> Create </button>            
        </form>
    )
}

export default CreateBulletin;
