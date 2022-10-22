import { createEffect, createSignal, For } from 'solid-js';
import axios from 'axios';
// import { User, UserDocument } from '@nx-go-playground/api/usesssrs';

function getUsers() {
  return (
    axios
      // .get<User[]>('http://localhost:8080/api/users')
      .get('/api/projects')
      .then((r) => r.data)
  );
}

const Predictions = () => {
  const [data, setData] = createSignal([]);
  createEffect(() => {
    getUsers().then((res) => {
      setData(res);
    });
  });
  return (
    <div>
      <h1>Predictions</h1>
      <div>
        data here
        <For each={data()}>
          {(item: any) => {
            return <div>{item.name}</div>;
          }}
        </For>
      </div>
    </div>
  );
};

export default Predictions;
