type Props = {
  visible: boolean;
};

export default function Loading({ visible }: Props) {
  return (
    <div className={"loading " + (visible ? 'active' : '')}>
      <h1>W</h1>
      <p>Word of the day</p>
    </div>
  );
}
