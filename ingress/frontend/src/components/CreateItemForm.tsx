import React, { useState, ChangeEvent } from 'react';
import { createToDoItemCall } from "../api/create";


interface CreateToDoItemProps {
    passBackResponse: (response: any) => void;
}


export const CreateToDoItem: React.FC<CreateToDoItemProps> = ({ passBackResponse }) => {
    const [title, setTitle] = useState<string>("");
    const handleTitleChange = (e: ChangeEvent<HTMLInputElement>
        ) => {
        setTitle(e.target.value);
    };
    const createItem = async () => {
        await createToDoItemCall(title).then(response => {
        setTitle("");
        if (response && response.data) {
            passBackResponse(response.data);
        } else if (response && response.error) {
            console.log(response);
            console.log(
                `Error ${response.status}: ${response.error}`
            );
        }
    });
    };
    return (
          <div className="inputContainer">
        <input type="text" id="name"
               placeholder="create to do item"
               value={title}
               onChange={handleTitleChange}/>
        <button className="actionButton"
             id="create-button"
             onClick={createItem}>Create</button>
    </div>
    );
}